#include <iostream>
#include <sstream>
#include <vector>
#include <memory>
#include <fstream>
#include <cassert>
using namespace std;

// TODO: user-friendly error messages

#define CHECK(expr)                                                     \
  if (expr) {                                                           \
  } else {                                                              \
    fprintf(stderr, "CHECK Failed (%s:%d): %s\n",                       \
            __FILE__, __LINE__, #expr);                                 \
    exit(EXIT_FAILURE);                                                 \
  }

#define CHECK_PERROR(expr)                                                     \
  if (expr) {                                                           \
  } else {                                                              \
    fprintf(stderr, "CHECK Failed (%s:%d): %s: ",                       \
            __FILE__, __LINE__, #expr);                                 \
    perror(nullptr); \
    exit(EXIT_FAILURE);                                                 \
  }

template<typename T>
void write_binary(std::ostream &os, const T &t) {
  CHECK_PERROR(os.write((char*)&t, sizeof(T)));
}


struct Direction {
  char dir;

  static Direction Parse(const string &token) {
    CHECK(token.length() == 1);
    char dir = tolower(token[0]);
    CHECK(dir == 'x' || dir == 'y' || dir == 'z');
    return Direction{ dir };
  }

  uint8_t ToBinary() {
    switch (dir) {
      case 'x': return 0b01;
      case 'y': return 0b10;
      case 'z': return 0b11;
    }
    assert(false);
  }
};

struct NearCoordinates {
  int x, y, z;

  static NearCoordinates Parse(const vector<string> &tokens, size_t token_offset) {
    NearCoordinates a;
    a.x = stoi(tokens[token_offset + 0]);
    a.y = stoi(tokens[token_offset + 1]);
    a.z = stoi(tokens[token_offset + 2]);
    CHECK(abs(a.x) + abs(a.y) + abs(a.z) <= 2);
    CHECK(max(max(abs(a.x), abs(a.y)), abs(a.z)) == 1);
    return a;
  }

  uint8_t ToBinary() {
    return (x + 1) * 9 + (y + 1) * 3 + (z + 1);
  }
};

//
// Base operation classes
//

struct Operation {
  virtual void Emit(ostream &os) = 0;
  virtual ~Operation() {}
};

template<class Derived>
struct NullaryOperation : public Operation {
  static Derived Parse(const vector<string> &tokens) {
    CHECK(tokens.size() == 1);
    return Derived();
  }

  virtual uint8_t GetSignature() = 0;

  virtual void Emit(ostream &os) {
    write_binary<uint8_t>(os, GetSignature());
  }
};

template<class Derived>
struct NCOnlyOperation : public Operation {
  NearCoordinates nc;

  static Derived Parse(const vector<string> &tokens) {
    CHECK(tokens.size() == 4);
    Derived a;
    a.nc = NearCoordinates::Parse(tokens, 1);
    return a;
  }

  virtual uint8_t GetSignature() = 0;

  virtual void Emit(ostream &os) {
    write_binary<uint8_t>(os, (nc.ToBinary() << 3) | GetSignature());
  }
};

//
// Concrete operation classes
//

struct Halt : public NullaryOperation<Halt> {
  virtual uint8_t GetSignature() {
    return 0b11111111;
  }
};

struct Wait : public NullaryOperation<Wait> {
  virtual uint8_t GetSignature() {
    return 0b11111110;
  }
};

struct Flip : public NullaryOperation<Flip> {
  virtual uint8_t GetSignature() {
    return 0b11111101;
  }
};

struct SMove : public Operation {
  Direction dir;
  int dis;

  static SMove Parse(const vector<string> &tokens) {
    CHECK(tokens.size() == 3);
    SMove a;
    a.dir = Direction::Parse(tokens[1]);
    a.dis = stoi(tokens[2]);
    CHECK(abs(a.dis) <= 15);
    return a;
  }

  virtual void Emit(ostream &os) {
    write_binary<uint8_t>(os, (dir.ToBinary() << 4) | 0b0100);
    write_binary<uint8_t>(os, dis + 15);
  }
};

struct LMove : public Operation {
  Direction dir1, dir2;
  int dis1, dis2;

  static LMove Parse(const vector<string> &tokens) {
    CHECK(tokens.size() == 5);
    LMove a;

    a.dir1 = Direction::Parse(tokens[1]);
    a.dis1 = stoi(tokens[2]);
    CHECK(abs(a.dis1) <= 5);

    a.dir2 = Direction::Parse(tokens[3]);
    a.dis2 = stoi(tokens[4]);
    CHECK(abs(a.dis2) <= 5);

    return a;
  }

  virtual void Emit(ostream &os) {
    write_binary<uint8_t>(os, (dir2.ToBinary() << 6) | (dir1.ToBinary() << 4) | 0b1100);
    write_binary<uint8_t>(os, ((dis2 + 5) << 4) | (dis1 + 5));
  }
};


struct FusionP : public NCOnlyOperation<FusionP> {
  virtual uint8_t GetSignature() {
    return 0b111;
  }
};

struct FusionS : public NCOnlyOperation<FusionS> {
  virtual uint8_t GetSignature() {
    return 0b110;
  }
};

struct Fission : Operation {
  NearCoordinates nc;
  int m;

  static Fission Parse(const vector<string> &tokens) {
    CHECK(tokens.size() == 5);
    Fission a;
    a.nc = NearCoordinates::Parse(tokens, 1);
    a.m = stoi(tokens[4]);
    return a;
  }

  virtual void Emit(ostream &os) {
    write_binary<uint8_t>(os, (nc.ToBinary() << 3) | 0b101);
    write_binary<uint8_t>(os, m);
  }
};

struct Fill : public NCOnlyOperation<Fill> {
  virtual uint8_t GetSignature() {
    return 0b011;
  }
};

//
// Main
//

template<class T> vector<T> Split(const string &str) {
  stringstream ss(str);
  vector<T> res;
  T t;
  while (ss >> t) res.push_back(t);
  return res;
}

Operation *ParseOperation(const string &line, size_t lineno) {
  if (line.length() >= 1 && line[0] == '#') return nullptr;

  vector<string> tokens = Split<string>(line);
  if (tokens.size() == 0) return nullptr;

  string &op = tokens[0];
  for (char &c: op) c = tolower(c);

  if (op == "halt") {
    return new Halt(Halt::Parse(tokens));
  } else if (op == "wait") {
    return new Wait(Wait::Parse(tokens));
  } else if (op == "flip") {
    return new Flip(Flip::Parse(tokens));
  } else if (op == "smove") {
    return new SMove(SMove::Parse(tokens));
  } else if (op == "lmove") {
    return new LMove(LMove::Parse(tokens));
  } else if (op == "fusionp") {
    return new FusionP(FusionP::Parse(tokens));
  } else if (op == "fusions") {
    return new FusionS(FusionS::Parse(tokens));
  } else if (op == "fission") {
    return new Fission(Fission::Parse(tokens));
  } else if (op == "fill") {
    return new Fill(Fill::Parse(tokens));
  } else {
    cerr << "Error at Line " << lineno << ": Unknown operation: " << op << endl;
  }
  return nullptr;
}

void ConvertAll(istream &is, ostream &os) {
  string line;
  for (int lineno = 0; getline(is, line); ++lineno) {
    auto op = ParseOperation(line, lineno);
    if (op) {
      op->Emit(os);
    }
  }
}

int main(int argc, char **argv) {
  CHECK(argc == 2);
  ofstream ofs(argv[1]);
  ios::sync_with_stdio(false);
  ConvertAll(cin, ofs);
}
