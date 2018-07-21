#include <array>
#include <bitset>
#include <cstdio>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>
#include "base/base.h"

DEFINE_string(p, "", "problem file (.mdl)");
DEFINE_string(a, "", "assembly file (.nbt)");

struct Coord {
  int x, y, z;
  bool operator==(const Coord& rhs) const {
    return x == rhs.x && y == rhs.y && z == rhs.z;
  }
  bool operator!=(const Coord& rhs) const { return !(*this == rhs); }
  bool is_valid(int r) const {
    return 0 <= x && x < r && 0 <= y && y < r && 0 <= z && z < r;
  }
  int& axis(int a) {
    switch (a) {
      case 1:
        return x;
      case 2:
        return y;
      case 3:
        return z;
    }
    LOG(FATAL) << "Bad axis";
  }
};
namespace std {
template <>
struct hash<Coord> {
  std::size_t operator()(const Coord& c) const {
    return (c.z * 251 + c.y) * 251 + c.x;
  }
};
}  // namespace std
static const Coord kZero = {0, 0, 0};

typedef Coord DCoord;

Coord operator+(const Coord& lhs, const DCoord& rhs) {
  return Coord{lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z};
}

bool check_range(int v, int c) { return 0 <= v && v < c; }

DCoord near_diff(int nd) {
  // nd == (dx + 1) * 9 + (dy + 1) * 3 + (dz + 1)
  DCoord d;
  d.z = nd % 3 - 1;
  nd /= 3;
  d.y = nd % 3 - 1;
  nd /= 3;
  d.x = nd - 1;
  if (d.x != 0 && d.y != 0 && d.z != 0) LOG(FATAL) << "invalid near difference";
  return d;
}

struct Matrix {
 public:
  Matrix(int r) : r_(r), m_(r * r * r, false) {}
  bool operator[](const Coord& c) const { return m_[index(c)]; }
  std::vector<bool>::reference operator[](const Coord& c) {
    return m_[index(c)];
  }

 private:
  size_t index(const Coord& c) const { return (c.z * r_ + c.y) * r_ + c.x; }

  int r_;
  std::vector<bool> m_;
};

struct Nanobot {
  Coord pos;
  std::vector<uint8> seeds;
  template <class It>
  Nanobot(const Coord& _pos, const It& begin, const It& end)
      : pos(_pos), seeds(begin, end) {}
  template <class C>
  Nanobot(const Coord& _pos, const C& seeds)
      : Nanobot(_pos, seeds.begin(), seeds.end()) {}
};

struct State {
  const int r;

  int64 energy_global = 0;
  int64 energy_local = 0;
  int64 energy_smove = 0;
  int64 energy_lmove = 0;
  int64 energy_fill = 0;
  bool harmonics = false;
  Matrix matrix;
  std::unordered_map<int, Nanobot> bots;
  int steps = 0;
  int commands = 0;

  State(int _r) : r(_r), matrix(_r) {
    bots.emplace(
        std::piecewise_construct, std::forward_as_tuple(0),
        std::forward_as_tuple(kZero, std::initializer_list<uint8>{
                                         1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
                                         13, 14, 15, 16, 17, 18, 19}));
  }
  int64 energy() const {
    return energy_global + energy_local + energy_smove + energy_lmove +
           energy_fill;
  }
  string execute(FILE* fa) {
    while (!bots.empty()) {
      std::vector<std::pair<int, Nanobot>> bots_activated;
      std::unordered_map<int, Coord> fusionP;
      std::unordered_map<Coord, int> fusionS;
      std::unordered_set<Coord> volat;
      if (harmonics) {
        energy_global += 30 * r * r * r;
      } else {
        energy_global += 3 * r * r * r;
      }
      energy_local += 20 * bots.size();
      for (int i = 0; i < 20; ++i) {
        auto it = bots.find(i);
        if (it == bots.end()) continue;
        Nanobot& bot = it->second;
        if (!volat.emplace(bot.pos).second) return "Error: Interference";
        int b = fgetc(fa);
        if (b == EOF) return "Unexpected EOF (command group)";
        if (b == 0b11111111) {  // Halt
          if (bot.pos != kZero) return "Error: Halt at non-zero coordinate";
          if (bots.size() != 1) return "Error: Halt with multiple bots";
          if (harmonics) return "Error: Halt with High harmonics";
          bots.clear();
          VLOG(2) << "Halt";
        } else if (b == 0b11111110) {  // Wait
          // Do nothing.
          VLOG(2) << "Wait";
        } else if (b == 0b11111101) {  // Flip
          harmonics = !harmonics;
          VLOG(2) << "Flip";
        } else if ((b & 0b11001111) == 0b00000100) {  // SMove
          int llda = (b & 0b00110000) >> 4;
          if (llda == 0) return "SMove bad axis encoding";
          int b2 = fgetc(fa);
          if (b2 == EOF) return "Unexpected EOF (SMove)";
          int lldi = b2 & 0b00011111;
          int& a = bot.pos.axis(llda);
          int lldsign = lldi - 15 < 0 ? -1 : 1;
          int lldlen = std::abs(lldi - 15);
          for (int i = 0; i < lldlen; ++i) {
            a += lldsign;
            if (matrix[bot.pos]) return "Error: SMove through Full voxel";
            if (!volat.emplace(bot.pos).second) return "Error: Interference";
          }
          if (!check_range(a, r)) return "Error: Invalid coordinate (SMove)";
          energy_smove += 2 * lldlen;
          VLOG(2) << "SMove " << llda << " " << lldi;
        } else if ((b & 0b00001111) == 0b00001100) {  // LMove
          int sld1a = (b & 0b00110000) >> 4;
          int sld2a = (b /* & 0b11000000 */) >> 6;
          int b2 = fgetc(fa);
          if (b2 == EOF) return "Unexpected EOF (LMove)";
          int sld1i = b2 & 0b00001111;
          int sld2i = (b2 /* & 0b11110000 */) >> 4;
          int& a1 = bot.pos.axis(sld1a);
          int& a2 = bot.pos.axis(sld2a);
          int sld1sign = sld1i - 5 < 0 ? -1 : 1;
          int sld1len = std::abs(sld1i - 5);
          int sld2sign = sld2i - 5 < 0 ? -1 : 1;
          int sld2len = std::abs(sld2i - 5);
          for (int i = 0; i < sld1len; ++i) {
            a1 += sld1sign;
            if (matrix[bot.pos]) return "Error: LMove through Full voxel";
            if (!volat.emplace(bot.pos).second) return "Error: Interference";
          }
          for (int i = 0; i < sld2len; ++i) {
            a2 += sld2sign;
            if (matrix[bot.pos]) return "Error: LMove through Full voxel";
            if (!volat.emplace(bot.pos).second) return "Error: Interference";
          }
          if (!check_range(a1, r)) return "Error: Invalid coordinate (LMove)";
          if (!check_range(a2, r)) return "Error: Invalid coordinate (LMove)";
          energy_lmove += 2 * (sld1len + 2 + sld2len);
          VLOG(2) << "LMove "
                  << " xyz"[sld1a] << sld1i << " "
                  << " xyz"[sld2a] << sld2i;
        } else if ((b & 0b00000111) == 0b00000111) {  // FusionP
          int nd = (b /* & 0b11111000 */) >> 3;
          fusionP.emplace(i, bot.pos + near_diff(nd));
          VLOG(2) << "FusionP " << nd;
        } else if ((b & 0b00000111) == 0b00000110) {  // FusionS
          int nd = (b /* & 0b11111000 */) >> 3;
          bot.pos + near_diff(nd);
          fusionS.emplace(bot.pos + near_diff(nd), i);
          VLOG(2) << "FusionS " << nd;
        } else if ((b & 0b00000111) == 0b00000101) {  // Fission
          if (bot.seeds.empty()) return "Error: Fission with no seeds";
          int nd = (b /* & 0b11111000 */) >> 3;
          int m = fgetc(fa);
          if (m == EOF) return "Unexpected EOF (Fission)";
          if (bot.seeds.size() < m + 1) return "Error: Fission lacking seeds";
          int j = bot.seeds[0];
          Coord c = bot.pos + near_diff(nd);
          if (!c.is_valid(r)) return "Error: Invalid coordinate (Fission)";
          if (!volat.emplace(c).second) return "Error: Interference";
          bots_activated.emplace_back(
              std::piecewise_construct, std::forward_as_tuple(j),
              std::forward_as_tuple(c, bot.seeds.begin() + 1,
                                    bot.seeds.begin() + 1 + m));
          bot.seeds.erase(bot.seeds.begin(), bot.seeds.begin() + 1 + m);
          // s.energy += 24;
          VLOG(2) << "Fission " << nd << " " << m;
        } else if ((b & 0b00000111) == 0b00000011) {  // Fill
          int nd = (b /* & 0b11111000 */) >> 3;
          Coord c = bot.pos + near_diff(nd);
          if (!c.is_valid(r)) return "Error: Invalid coordinate (Fill)";
          if (!volat.emplace(c).second) return "Error: Interference";
          if (matrix[c]) {
            energy_fill += 6;
          } else {
            matrix[c] = true;
            energy_fill += 12;
          }
          VLOG(2) << "Fill " << nd;
        } else {
          LOG(ERROR) << "Unknown command at " << ftell(fa) - 1;
        }
        ++commands;
      }
      for (auto& p : fusionP) {
        Nanobot& primary = bots.find(p.first)->second;
        auto it = fusionS.find(primary.pos);
        if (it == fusionS.end())
          return "Error: FusionP with no matching FusionS";
        Nanobot& secondary = bots.find(it->second)->second;
        if (p.second != secondary.pos)
          return "Error: FusionP with no matching FusionS";
        std::vector<uint8> merged(primary.seeds.size() +
                                  secondary.seeds.size());
        std::merge(primary.seeds.begin(), primary.seeds.end(),
                   secondary.seeds.begin(), secondary.seeds.end(),
                   merged.begin());
        primary.seeds = std::move(merged);
        bots.erase(it->second);
        // s.energy -= 24;
        fusionS.erase(it);
      }
      if (!fusionS.empty()) return "Error: FusionS with no matching FusionP";
      for (auto& bot : bots_activated) bots.emplace(std::move(bot));
      // TODO: check volatile coordinates
      ++steps;
    }
    return "";
  }
};

int main(int argc, char** argv) {
  ParseCommandLineFlags(&argc, &argv);
  FILE* fp = fopen(FLAGS_p.c_str(), "r");
  LOG_IF(FATAL, fp == nullptr) << "Failed to read " << FLAGS_p;
  FILE* fa = fopen(FLAGS_a.c_str(), "r");
  LOG_IF(FATAL, fp == nullptr) << "Failed to read " << FLAGS_a;

  int r = fgetc(fp);
  // TODO: read model

  State s(r);
  string msg = s.execute(fa);
  LOG_IF(ERROR, !msg.empty()) << msg;
  LOG(INFO) << "Result:"
            << "\n           time : " << s.steps
            << "\n       commands : " << s.commands
            << "\n  global energy : " << s.energy_global
            << "\n   local energy : " << s.energy_local
            << "\n   smove energy : " << s.energy_smove
            << "\n   lmove energy : " << s.energy_lmove
            << "\n    fill energy : " << s.energy_fill
            << "\n \x1b[33m[ total energy : " << s.energy() << " ]\x1b[0m";
  // TODO: check matrix

  if (fp) fclose(fp);
  if (fa) fclose(fa);

  return msg.empty() ? 0 : 1;
}
