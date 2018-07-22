#include <array>
#include <cstdio>
#include <memory>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>
#include "base/base.h"
#include "strings/join.h"

DEFINE_string(s, "", "srouce model file (.nbt)");
DEFINE_string(t, "", "target model file (.nbt)");
DEFINE_string(p, "", "(deprecated)");
DEFINE_int32(r, 0, "R instead of problem; no model check");
DEFINE_string(a, "", "(deprecated)");
DEFINE_string(output_model, "tmp.mdl", "output final state as model");

struct Coord {
  int x, y, z;
  Coord(int _x, int _y, int _z) : x(_x), y(_y), z(_z) {}
  Coord(const Coord& c) : x(c.x), y(c.y), z(c.z) {}
  bool operator==(const Coord& rhs) const {
    return x == rhs.x && y == rhs.y && z == rhs.z;
  }
  bool operator!=(const Coord& rhs) const { return !(*this == rhs); }
  Coord operator+(const Coord& rhs) const {
    return Coord{x + rhs.x, y + rhs.y, z + rhs.z};
  }
  bool is_valid(int r) const {
    return 0 <= x && x < r && 0 <= y && y < r && 0 <= z && z < r;
  }
};
typedef Coord DCoord;
static const Coord kZero = {0, 0, 0};
static const std::array<DCoord, 6> kAxis = {{
    DCoord{1, 0, 0},
    DCoord{0, 1, 0},
    DCoord{0, 0, 1},
    DCoord{-1, 0, 0},
    DCoord{0, -1, 0},
    DCoord{0, 0, -1},
}};

ostream& operator<<(ostream& os, const Coord& c) {
  return os << '<' << c.x << ',' << c.y << ',' << c.z << '>';
}

struct Region {
  Coord a, b;
  Region(const Coord& _a, const Coord& _b) : a(_a), b(_b) {
    if (a.x > b.x) std::swap(a.x, b.x);
    if (a.y > b.y) std::swap(a.y, b.y);
    if (a.z > b.z) std::swap(a.z, b.z);
  }
  Region(const Region& r) : a(r.a), b(r.b) {}
  bool operator==(const Region& rhs) const { return a == rhs.a && b == rhs.b; }
  bool is_valid() const { return a != b; }
  int dimension() const {
    return (a.x == b.x ? 0 : 1) + (a.y == b.y ? 0 : 1) + (a.z == b.z ? 0 : 1);
  }
  std::vector<Coord> members() const {
    std::vector<Coord> v;
    for (int x = a.x; x <= b.x; ++x) {
      for (int y = a.y; y <= b.y; ++y) {
        for (int z = a.z; z <= b.z; ++z) {
          v.emplace_back(x, y, z);
        }
      }
    }
    return v;
  }
};

namespace std {
template <>
struct hash<Coord> {
  std::size_t operator()(const Coord& c) const {
    return (c.z * 251 + c.y) * 251 + c.x;
  }
};
template <>
struct hash<Region> {
  std::size_t operator()(const Region& r) const {
    hash<Coord> h;
    return h(r.a) * 15813257 + h(r.b);
  }
};
}  // namespace std

bool check_range(int v, int c) { return 0 <= v && v < c; }

DCoord near_diff(int nd) {
  // nd == (dx + 1) * 9 + (dy + 1) * 3 + (dz + 1)
  int z = nd % 3 - 1;
  nd /= 3;
  int y = nd % 3 - 1;
  nd /= 3;
  int x = nd - 1;
  if (x != 0 && y != 0 && z != 0) LOG(FATAL) << "invalid near difference";
  return DCoord(x, y, z);
}

struct Matrix {
 public:
  Matrix(int r) : r_(r), m_(r * r * r, false) {}
  template <class C>
  Matrix(int r, const C& bytes) : r_(r), m_(r * r * r) {
    for (int i = 0; i < bytes.size(); ++i) {
      for (int j = 0; j < 8; ++j) {
        if (i * 8 + j >= m_.size()) return;
        m_[i * 8 + j] = bytes[i] >> j & 1;
      }
    }
  }
  Matrix(const Matrix& m) : r_(m.r_), m_(m.m_) {}
  int r() const { return r_; }
  bool operator[](const Coord& c) const { return m_[index(c)]; }
  std::vector<bool>::reference operator[](const Coord& c) {
    return m_[index(c)];
  }
  bool operator==(const Matrix& rhs) const { return m_ == rhs.m_; }
  bool operator!=(const Matrix& rhs) const { return m_ != rhs.m_; }
  std::vector<uint8> dump() const {
    std::vector<uint8> bytes((m_.size() + 7) / 8, 0);
    for (int i = 0; i < bytes.size(); ++i) {
      for (int j = 0; j < 8; ++j) {
        if (i * 8 + j < m_.size() && m_[i * 8 + j]) bytes[i] |= 1 << j;
      }
    }
    return bytes;
  }

 private:
  size_t index(const Coord& c) const { return (c.x * r_ + c.y) * r_ + c.z; }

  const int r_;
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
  int64 energy_void = 0;
  bool harmonics = false;
  Matrix matrix;
  std::map<int, Nanobot> bots;
  int steps = 0;
  int commands = 0;

  State(int _r) : State(Matrix(_r)) {}
  State(const Matrix& m) : r(m.r()), matrix(m) {
    vector<uint8> seeds(39);
    for (int i = 0; i < 39; ++i) seeds[i] = i + 1;
    bots.emplace(std::piecewise_construct, std::forward_as_tuple(0),
                 std::forward_as_tuple(kZero, seeds));
  }
  int64 energy() const {
    return energy_global + energy_local + energy_smove + energy_lmove +
           energy_fill + energy_void;
  }
  bool execute(FILE* fa) {
    while (!bots.empty()) {
      std::vector<Coord> filled;
      std::vector<Coord> voided;
      std::vector<std::pair<int, Nanobot>> bots_activated;
      std::unordered_map<int, Coord> fusionP;
      std::unordered_map<Coord, int> fusionS;
      std::unordered_map<Region, std::unordered_set<Coord>> gfilled;
      std::unordered_map<Region, std::unordered_set<Coord>> gvoided;
      std::unordered_set<Coord> volat;
      bool halted = false;

      energy_global += harmonics ? 30 * r * r * r : 3 * r * r * r;
      energy_local += 20 * bots.size();
      VLOG(3) << "time " << steps;
      for (auto& p : bots) {
        int i = p.first;
        Nanobot& bot = p.second;
        VLOG(3) << "bot[" << i << "] " << bot.pos
                << " seeds:" << strings::JoinInts(bot.seeds, ",");
        if (!check_interference(&volat, bot.pos)) return false;
        int b = fgetc(fa);
        if (!check_eof(fa)) return false;
        if (b == 0b11111111) {  // Halt
          VLOG(2) << "Halt";
          if (bot.pos != kZero) {
            LOG(ERROR) << "Halt at non-zero coordinate";
            return false;
          }
          if (bots.size() != 1) {
            LOG(ERROR) << "Halt with multiple bots";
            return false;
          }
          if (harmonics) {
            LOG(ERROR) << "Halt with High harmonics";
            return false;
          }
          halted = true;
        } else if (b == 0b11111110) {  // Wait
          VLOG(2) << "Wait";
        } else if (b == 0b11111101) {  // Flip
          VLOG(2) << "Flip";
          harmonics = !harmonics;
        } else if ((b & 0b11001111) == 0b00000100) {  // SMove
          int llda = (b & 0b00110000) >> 4;
          int b2 = fgetc(fa);
          if (!check_eof(fa)) return false;
          int lldi = (b2 & 0b00011111) - 15;
          VLOG(2) << "SMove "
                  << "?xyz"[llda] << " " << lldi;
          if (llda == 0 || lldi == 0 || lldi < -15 || 15 < lldi) {
            LOG(ERROR) << "Invalid long linear coordinate difference";
            return false;
          }
          DCoord dir = kAxis[lldi < 0 ? llda - 1 + 3 : llda - 1];
          int lldlen = std::abs(lldi);
          for (int i = 0; i < lldlen; ++i) {
            bot.pos = bot.pos + dir;
            if (!check_coord(bot.pos)) return false;
            if (matrix[bot.pos]) {
              LOG(ERROR) << "SMove through Full voxel " << bot.pos;
              return false;
            }
            if (!check_interference(&volat, bot.pos)) return false;
          }
          energy_smove += 2 * lldlen;
        } else if ((b & 0b00001111) == 0b00001100) {  // LMove
          int sld1a = (b & 0b00110000) >> 4;
          int sld2a = (b /* & 0b11000000 */) >> 6;
          int b2 = fgetc(fa);
          if (!check_eof(fa)) return false;
          int sld1i = (b2 & 0b00001111) - 5;
          int sld2i = ((b2 /* & 0b11110000 */) >> 4) - 5;
          VLOG(2) << "LMove "
                  << "?xyz"[sld1a] << " " << sld1i << " "
                  << "?xyz"[sld2a] << " " << sld2i;
          if (sld1a == 0 || sld1i == 0 || sld1i < -5 || 5 < sld1i ||
              sld2a == 0 || sld2i == 0 || sld2i < -5 || 5 < sld2i) {
            LOG(ERROR) << "Invalid short linear coordinate difference";
            return false;
          }
          int sld1len = std::abs(sld1i);
          int sld2len = std::abs(sld2i);
          DCoord dir1 = kAxis[sld1i < 0 ? sld1a - 1 + 3 : sld1a - 1];
          DCoord dir2 = kAxis[sld2i < 0 ? sld2a - 1 + 3 : sld2a - 1];
          for (int i = 0; i < sld1len; ++i) {
            bot.pos = bot.pos + dir1;
            if (!check_coord(bot.pos)) return false;
            if (matrix[bot.pos]) {
              LOG(ERROR) << "LMove through Full voxel " << bot.pos;
              return false;
            }
            if (!check_interference(&volat, bot.pos)) return false;
          }
          for (int i = 0; i < sld2len; ++i) {
            bot.pos = bot.pos + dir2;
            if (!check_coord(bot.pos)) return false;
            if (matrix[bot.pos]) {
              LOG(ERROR) << "LMove through Full voxel " << bot.pos;
              return false;
            }
            if (!check_interference(&volat, bot.pos)) return false;
          }
          energy_lmove += 2 * (sld1len + 2 + sld2len);
        } else if ((b & 0b00000111) == 0b00000111) {  // FusionP
          int nd = (b /* & 0b11111000 */) >> 3;
          DCoord dc = near_diff(nd);
          VLOG(2) << "FusionP " << dc;
          fusionP.emplace(i, bot.pos + dc);
        } else if ((b & 0b00000111) == 0b00000110) {  // FusionS
          int nd = (b /* & 0b11111000 */) >> 3;
          DCoord dc = near_diff(nd);
          VLOG(2) << "FusionS " << dc;
          fusionS.emplace(bot.pos + dc, i);
        } else if ((b & 0b00000111) == 0b00000101) {  // Fission
          if (bot.seeds.empty()) {
            LOG(ERROR) << "Fission with no seeds";
            return false;
          }
          int nd = (b /* & 0b11111000 */) >> 3;
          int m = fgetc(fa);
          if (!check_eof(fa)) return false;
          if (bot.seeds.size() < m + 1) {
            LOG(ERROR) << "Fission lacking seeds";
            return false;
          }
          int j = bot.seeds[0];
          DCoord dc = near_diff(nd);
          Coord c = bot.pos + dc;
          VLOG(2) << "Fission " << dc << " " << m;
          if (!check_coord(c)) return false;
          if (!check_interference(&volat, c)) return false;
          bots_activated.emplace_back(
              std::piecewise_construct, std::forward_as_tuple(j),
              std::forward_as_tuple(c, bot.seeds.begin() + 1,
                                    bot.seeds.begin() + 1 + m));
          bot.seeds.erase(bot.seeds.begin(), bot.seeds.begin() + 1 + m);
          // s.energy += 24;
        } else if ((b & 0b00000111) == 0b00000011) {  // Fill
          int nd = (b /* & 0b11111000 */) >> 3;
          DCoord dc = near_diff(nd);
          Coord c = bot.pos + dc;
          VLOG(2) << "Fill " << dc;
          if (!check_coord(c)) return false;
          if (!check_interference(&volat, c)) return false;
          filled.push_back(c);
        } else if ((b & 0b00000111) == 0b00000010) {  // Void
          int nd = (b /* & 0b11111000 */) >> 3;
          DCoord dc = near_diff(nd);
          VLOG(2) << "Void " << dc;
          Coord c = bot.pos + dc;
          if (!check_coord(c)) return false;
          if (!check_interference(&volat, c)) return false;
          voided.push_back(c);
        } else if ((b & 0b00000111) == 0b00000001) {  // GFill
          int nd = (b /* & 0b11111000 */) >> 3;
          DCoord dc = near_diff(nd);
          int fdx = fgetc(fa);
          int fdy = fgetc(fa);
          int fdz = fgetc(fa);
          if (!check_eof(fa)) return false;
          DCoord fd(fdx - 30, fdy - 30, fdz - 30);
          VLOG(2) << "GFill " << dc << " " << fd;
          if (fd.x > 30 || fd.y > 30 || fd.z > 30) {
            LOG(ERROR) << "Invalid far coordinate distance " << fd;
            return false;
          }
          Coord c1 = bot.pos + dc;
          Coord c2 = c1 + fd;
          if (!check_coord(c1)) return false;
          if (!check_coord(c2)) return false;
          if (!gfilled
                   .emplace(std::piecewise_construct,
                            std::forward_as_tuple(c1, c2),
                            std::forward_as_tuple())
                   .first->second.emplace(c1)
                   .second) {
            LOG(ERROR) << "GFill corner conflict";
            return false;
          }
        } else if ((b & 0b00000111) == 0b00000000) {  // GVoid
          int nd = (b /* & 0b11111000 */) >> 3;
          DCoord dc = near_diff(nd);
          int fdx = fgetc(fa);
          int fdy = fgetc(fa);
          int fdz = fgetc(fa);
          if (!check_eof(fa)) return false;
          DCoord fd(fdx - 30, fdy - 30, fdz - 30);
          VLOG(2) << "GVoid " << dc << " " << fd;
          if (fd.x > 30 || fd.y > 30 || fd.z > 30) {
            LOG(ERROR) << "Invalid far coordinate distance " << fd;
            return false;
          }
          Coord c1 = bot.pos + dc;
          Coord c2 = c1 + fd;
          if (!check_coord(c1)) return false;
          if (!check_coord(c2)) return false;
          if (!gvoided
                   .emplace(std::piecewise_construct,
                            std::forward_as_tuple(c1, c2),
                            std::forward_as_tuple())
                   .first->second.emplace(c1)
                   .second) {
            LOG(ERROR) << "GVoid corner conflict";
            return false;
          }
        } else {
          LOG(ERROR) << "Unknown command";
          return false;
        }
        ++commands;
      }
      // Group commands
      for (auto& p : fusionP) {
        Nanobot& primary = bots.find(p.first)->second;
        auto it = fusionS.find(primary.pos);
        if (it == fusionS.end()) {
          LOG(ERROR) << "FusionP with no matching FusionS";
          return false;
        }
        Nanobot& secondary = bots.find(it->second)->second;
        if (p.second != secondary.pos) {
          LOG(ERROR) << "FusionP with no matching FusionS";
          return false;
        }
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
      if (!fusionS.empty()) {
        LOG(ERROR) << "FusionS with no matching FusionP";
        return false;
      }
      for (const auto& p : gfilled) {
        int dim = p.first.dimension();
        if (dim == 0) {
          LOG(ERROR) << "GFill 0-dimension";
          return false;
        }
        if (p.second.size() != (1 << dim)) {
          LOG(ERROR) << "Gfill mising corner";
          return false;
        }
        for (auto& m : p.first.members()) {
          if (!check_interference(&volat, m)) return false;
          filled.push_back(std::move(m));
        }
      }
      for (auto& p : gvoided) {
        int dim = p.first.dimension();
        if (dim == 0) {
          LOG(ERROR) << "GVoid 0-dimension";
          return false;
        }
        if (p.second.size() != (1 << dim)) {
          LOG(ERROR) << "GVoid mising corner";
          return false;
        }
        for (auto& m : p.first.members()) {
          if (!check_interference(&volat, m)) return false;
          voided.push_back(std::move(m));
        }
      }

      for (const Coord& c : filled) energy_fill += matrix[c] ? 6 : 12;
      for (const Coord& c : voided) energy_void += matrix[c] ? -12 : 3;

      for (const Coord& c : voided) matrix[c] = false;
      // Check grounding
      if (!harmonics) {
        // connections
        std::unordered_set<Coord> uncon(filled.begin(), filled.end());
        while (!uncon.empty()) {
          auto it = uncon.begin();
          Coord u = *it;
          uncon.erase(it);
          static const auto k1N1 = {
              DCoord{-1, 0, 0}, DCoord{0, -1, 0}, DCoord{0, 0, -1},
              DCoord{1, 0, 0},  DCoord{0, 1, 0},  DCoord{0, 0, 1},
          };
          if (std::none_of(k1N1.begin(), k1N1.end(), [&](const DCoord& d) {
                Coord v = u + d;
                return v.y == 0 || matrix[v] || uncon.count(v);
              })) {
            LOG(ERROR) << "Ungrounded Full voxel " << u;
            return false;
          }
          matrix[u] = true;
        }
        // disconnections
        std::unordered_set<Coord> g;  // known to be grounded
        for (const Coord& v : voided) {
          for (const DCoord& a : kAxis) {
            Coord c = v + a;  // each 6-neighbor to the hole
            if (c.is_valid(r) && matrix[c]) {
              std::vector<Coord> s(1, c);  // bfs stack to the ground
              std::unordered_set<Coord> t(s.begin(), s.end());
              while (!s.empty()) {
                Coord u = s.back();
                if (u.y == 0 || g.count(u) > 0) {
                  g.insert(t.begin(), t.end());
                  break;
                }
                s.pop_back();
                for (const DCoord& b : kAxis) {
                  Coord d = u + b;
                  if (d.is_valid(r) && matrix[d] && t.insert(d).second) {
                    s.push_back(d);
                  }
                }
              }
              if (s.empty()) {
                LOG(ERROR) << "Ungrounded by Void voxel";
                return false;
              }
            }
          }
        }
      }
      for (const Coord& c : filled) matrix[c] = true;
      for (auto& bot : bots_activated) bots.emplace(std::move(bot));
      if (halted) bots.clear();
      ++steps;
    }
    return true;
  }
  static bool check_eof(FILE* fp) {
    if (feof(fp)) {
      LOG(ERROR) << "Unexpected EOF";
      return false;
    }
    return true;
  }
  bool check_coord(const Coord& c) const {
    if (!c.is_valid(r)) {
      LOG(ERROR) << "Invalid coordinate " << c;
      return false;
    }
    return true;
  }
  bool check_interference(std::unordered_set<Coord>* v, const Coord& c) const {
    if (!v->emplace(c).second) {
      LOG(ERROR) << "Interference " << c;
      return false;
    }
    return true;
  }
};

std::unique_ptr<Matrix> read_model(const char* filename) {
  FILE* fp = fopen(filename, "r");
  CHECK(fp != nullptr) << "Failed to open " << filename;
  int r = fgetc(fp);
  CHECK_LE(r, 250);
  std::vector<uint8> buf((r * r * r + 7) / 8);
  CHECK(fread(buf.data(), buf.size(), 1, fp) == 1)
      << "Failed to read " << filename;
  fclose(fp);
  return std::unique_ptr<Matrix>(new Matrix(r, buf));
}

void write_model(const Matrix& m, const char* filename) {
  FILE* fp = fopen(filename, "w");
  CHECK(fp != nullptr) << "Failed to open " << filename;
  fputc(m.r(), fp);
  vector<uint8> buf = m.dump();
  CHECK(fwrite(buf.data(), buf.size(), 1, fp) == 1)
      << "Failed to write " << filename;
  fclose(fp);
  LOG(INFO) << "Written " << filename;
}

int main(int argc, char** argv) {
  ParseCommandLineFlags(&argc, &argv);

  // compat
  if (!FLAGS_p.empty()) FLAGS_t = FLAGS_p;
  const char* tracefile = argc > 1 ? argv[1] : FLAGS_a.c_str();
  CHECK(tracefile != nullptr)
      << "Specify trace.nbt [ -s source.mdl ] [ -t target.mdl ]";

  int r = FLAGS_r;
  std::unique_ptr<Matrix> source, target;
  if (!FLAGS_t.empty()) {
    target = read_model(FLAGS_t.c_str());
    r = target->r();
  }
  if (!FLAGS_s.empty()) {
    source = read_model(FLAGS_s.c_str());
    if (target) {
      CHECK_EQ(source->r(), target->r());
    } else {
      r = source->r();
    }
  } else {
    source = std::unique_ptr<Matrix>(new Matrix(r));
  }
  CHECK_GT(r, 0);

  FILE* fa = fopen(tracefile, "r");
  CHECK(fa != nullptr) << "Failed to read " << tracefile;

  State s(*source);
  bool success = s.execute(fa);
  LOG_IF(INFO, success) << "Halted successfully";
  LOG(INFO) << "Result:"
            << "\n           time : " << s.steps
            << "\n       commands : " << s.commands
            << "\n  global energy : " << s.energy_global
            << "\n   local energy : " << s.energy_local
            << "\n   smove energy : " << s.energy_smove
            << "\n   lmove energy : " << s.energy_lmove
            << "\n    fill energy : " << s.energy_fill
            << "\n \x1b[33m[ total energy : " << s.energy() << " ]\x1b[0m";
  if (success) {
    printf("time:%d\n", s.steps);
    printf("commands:%d\n", s.commands);
    printf("energy:%lld\n", s.energy());
  }

  if (!FLAGS_output_model.empty()) {
    write_model(s.matrix, FLAGS_output_model.c_str());
  }

  if (!success) return 1;

  if (target && s.matrix != *target) {
    LOG(ERROR) << "Constructed model unmatched";
    return 1;
  }

  return 0;
}
