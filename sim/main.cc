#include <array>
#include <cstdio>
#include <memory>
#include <string>
#include <string_view>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>
#include "base/base.h"

DEFINE_string(p, "", "problem file (.mdl)");
DEFINE_int32(r, 0, "R instead of problem; no model check");
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

ostream& operator<<(ostream& os, const Coord& c) {
  return os << '<' << c.x << ',' << c.y << ',' << c.z << '>';
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
  template <class C>
  Matrix(int r, const C& bytes) : r_(r), m_(r * r * r) {
    for (int i = 0; i < bytes.size(); ++i) {
      for (int j = 0; j < 8; ++j) {
        if (i * 8 + j >= m_.size()) return;
        m_[i * 8 + j] = bytes[i] >> j & 1;
      }
    }
  }
  bool operator[](const Coord& c) const { return m_[index(c)]; }
  std::vector<bool>::reference operator[](const Coord& c) {
    return m_[index(c)];
  }
  bool operator==(const Matrix& rhs) const { return m_ == rhs.m_; }
  bool operator!=(const Matrix& rhs) const { return m_ != rhs.m_; }

 private:
  size_t index(const Coord& c) const { return (c.x * r_ + c.y) * r_ + c.z; }

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
  bool execute(FILE* fa) {
    while (!bots.empty()) {
      std::vector<std::pair<int, Nanobot>> bots_activated;
      std::unordered_map<int, Coord> fusionP;
      std::unordered_map<Coord, int> fusionS;
      std::unordered_set<Coord> volat;
      std::unordered_set<Coord> uncon;
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
        if (!volat.emplace(bot.pos).second) {
          LOG(ERROR) << "Interference " << bot.pos;
          return false;
        }
        int b = fgetc(fa);
        if (b == EOF) {
          LOG(ERROR) << "Unexpected EOF (command group)";
          return false;
        }
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
          bots.clear();
        } else if (b == 0b11111110) {  // Wait
          VLOG(2) << "Wait";
        } else if (b == 0b11111101) {  // Flip
          VLOG(2) << "Flip";
          harmonics = !harmonics;
        } else if ((b & 0b11001111) == 0b00000100) {  // SMove
          int llda = (b & 0b00110000) >> 4;
          if (llda == 0) {
            LOG(ERROR) << "SMove bad axis encoding";
            return false;
          }
          int b2 = fgetc(fa);
          if (b2 == EOF) {
            LOG(ERROR) << "Unexpected EOF (SMove)";
            return false;
          }
          int lldi = (b2 & 0b00011111) - 15;
          VLOG(2) << "SMove "
                  << " xyz"[llda] << " " << lldi;
          LOG_IF(FATAL, lldi == 0 || lldi < -15 || 15 < lldi)
              << "Invalid long linear coordinate difference";
          int& a = bot.pos.axis(llda);
          int lldsign = lldi < 0 ? -1 : 1;
          int lldlen = std::abs(lldi);
          for (int i = 0; i < lldlen; ++i) {
            a += lldsign;
            if (matrix[bot.pos]) {
              LOG(ERROR) << "SMove through Full voxel " << bot.pos;
              return false;
            }
            if (!volat.emplace(bot.pos).second) {
              LOG(ERROR) << "Interference " << bot.pos;
              return false;
            }
          }
          if (!check_range(a, r)) {
            LOG(ERROR) << "Invalid coordinate (SMove)";
            return false;
          }
          energy_smove += 2 * lldlen;
        } else if ((b & 0b00001111) == 0b00001100) {  // LMove
          int sld1a = (b & 0b00110000) >> 4;
          int sld2a = (b /* & 0b11000000 */) >> 6;
          int b2 = fgetc(fa);
          if (b2 == EOF) {
            LOG(ERROR) << "Unexpected EOF (LMove)";
            return false;
          }
          int sld1i = (b2 & 0b00001111) - 5;
          int sld2i = ((b2 /* & 0b11110000 */) >> 4) - 5;
          VLOG(2) << "LMove "
                  << " xyz"[sld1a] << " " << sld1i << " "
                  << " xyz"[sld2a] << " " << sld2i;
          LOG_IF(FATAL, sld1i == 0 || sld1i < -5 || 5 < sld1i)
              << "Invalid short linear coordinate difference";
          LOG_IF(FATAL, sld2i == 0 || sld2i < -5 || 5 < sld2i)
              << "Invalid short linear coordinate difference";
          int& a1 = bot.pos.axis(sld1a);
          int& a2 = bot.pos.axis(sld2a);
          int sld1sign = sld1i < 0 ? -1 : 1;
          int sld1len = std::abs(sld1i);
          int sld2sign = sld2i < 0 ? -1 : 1;
          int sld2len = std::abs(sld2i);
          for (int i = 0; i < sld1len; ++i) {
            a1 += sld1sign;
            if (matrix[bot.pos]) {
              LOG(ERROR) << "LMove through Full voxel " << bot.pos;
              return false;
            }
            if (!volat.emplace(bot.pos).second) {
              LOG(ERROR) << "Interference " << bot.pos;
              return false;
            }
          }
          for (int i = 0; i < sld2len; ++i) {
            a2 += sld2sign;
            if (matrix[bot.pos]) {
              LOG(ERROR) << "LMove through Full voxel " << bot.pos;
              return false;
            }
            if (!volat.emplace(bot.pos).second) {
              LOG(ERROR) << "Interference " << bot.pos;
              return false;
            }
          }
          if (!check_range(a1, r) || !check_range(a2, r)) {
            LOG(ERROR) << "Invalid coordinate (LMove)";
            return false;
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
          if (m == EOF) {
            LOG(ERROR) << "Unexpected EOF (Fission)";
            return false;
          }
          if (bot.seeds.size() < m + 1) {
            LOG(ERROR) << "Fission lacking seeds";
            return false;
          }
          int j = bot.seeds[0];
          DCoord dc = near_diff(nd);
          Coord c = bot.pos + dc;
          VLOG(2) << "Fission " << dc << " " << m;
          if (!c.is_valid(r)) {
            LOG(ERROR) << "Invalid coordinate (Fission)";
            return false;
          }
          if (!volat.emplace(c).second) {
            LOG(ERROR) << "Interference " << c;
            return false;
          }
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
          if (!c.is_valid(r)) {
            LOG(ERROR) << "Invalid coordinate (Fill)";
            return false;
          }
          if (!volat.emplace(c).second) {
            LOG(ERROR) << "Interference " << c;
            return false;
          }
          if (matrix[c]) {
            energy_fill += 6;
          } else {
            uncon.insert(c);
            energy_fill += 12;
          }
        } else {
          LOG(ERROR) << "Unknown command";
          return false;
        }
        ++commands;
      }
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
      for (auto& bot : bots_activated) bots.emplace(std::move(bot));
      if (harmonics) {
        for (const Coord& u : uncon) matrix[u] = true;
      } else {
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
                return matrix[v] || uncon.count(v);
              })) {
            LOG(ERROR) << "Ungrounded Full voxel " << u;
            return false;
          }
        }
      }
      ++steps;
    }
    return true;
  }
};

int main(int argc, char** argv) {
  ParseCommandLineFlags(&argc, &argv);
  LOG_IF(FATAL, FLAGS_a.empty() || FLAGS_p.empty() && FLAGS_r == 0)
      << "Specify -a *.nbt -p *.mdl";

  int r = FLAGS_r;
  std::unique_ptr<Matrix> model;
  if (!FLAGS_p.empty()) {
    FILE* fp = fopen(FLAGS_p.c_str(), "r");
    LOG_IF(FATAL, fp == nullptr) << "Failed to read " << FLAGS_p;
    r = fgetc(fp);
    std::vector<uint8> buf((r * r * r + 7) / 8);
    if (std::fread(buf.data(), buf.size(), buf.size(), fp) == 1) {
      model = std::unique_ptr<Matrix>(new Matrix(r, buf));
    } else {
      LOG(ERROR) << "Failed to read " << FLAGS_p;
    }
    fclose(fp);
  }

  FILE* fa = fopen(FLAGS_a.c_str(), "r");
  LOG_IF(FATAL, fa == nullptr) << "Failed to read " << FLAGS_a;

  State s(r);
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
  printf("time:%d\n", s.steps);
  printf("commands:%d\n", s.commands);
  printf("energy:%lld\n", s.energy());

  if (!success) return 1;

  LOG_IF(WARNING, !model) << "No model check";
  if (model && s.matrix != *model) {
    LOG(ERROR) << "Constructed model unmatched";
    return 1;
  }

  return 0;
}
