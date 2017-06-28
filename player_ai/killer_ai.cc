// ひたすら爆弾をとりに行くAI
#include <iostream>
#include <vector>
#include <string>
#include <queue>
#include <tuple>
using namespace std;

const bool DEBUG = false;

namespace Actions {
void move(int id) {
    cout << 1 << endl;
    cout << id << endl;
    if (DEBUG)
        cerr << 1 << ' ' << id << ">_<" << endl;
}
void nop() {
    cout << 0 << endl;
    if (DEBUG)
        cerr << 0 << " >_<" << endl;
}
void pick_bom() {
    cout << 2 << endl;
    if (DEBUG)
        cerr << 2 << " >_<" << endl;
}
void drop_bom() {
    cout << 3 << endl;
    if (DEBUG)
        cerr << 3 << " >_<" << endl;
}
}
// 下左右上
const int DX[4] = {0, -1, 1, 0};
const int DY[4] = {-1, 0, 0, 1};
const int INF = 1e9;
enum struct FieldState {
    None,
    Galon,
    BomReady,
    BomSafe,
};
struct FieldVal {
    FieldState type;
    int val;
    explicit FieldVal(): type(FieldState::None), val(0) {}
};

enum struct DistType {
    OtherPlayer,
    Bom,
};
struct GameInfo {
    vector<vector<FieldVal>> field;
    vector<pair<int, int>> bom_list;
    vector<pair<int, int>> others_list;
    vector<vector<bool>> danger;
    int size;
    int galon;
    int px;
    int py;
    int player_num;
    int my_id;
    int bom_period;
    explicit GameInfo()
            : field(), size(0), galon(0), px(0), py(0), player_num(0), bom_period(-1) {}
    void action(int s) {
        size = s;
        field.assign(s, vector<FieldVal>(s));
        danger.assign(s, vector<bool>(s));
        cin >> player_num >> my_id;
        others_list.clear();
        for (int i = 0; i < player_num; ++i) {
            int x, y, id;
            cin >> x >> y >> id;
            if (i == my_id) {
                px = x;
                py = y;
            } else {
                others_list.emplace_back(x, y);
            }
        }
        int oil_num, bom_num;
        cin >> oil_num;
        for (int i = 0; i < oil_num; ++i) {
            int x, y, g;
            cin >> x >> y >> g;
            field[y][x].type = FieldState::Galon;
            field[y][x].val = g;
        }
        cin >> bom_num;
        bom_list.clear();
        for (int i = 0; i < bom_num; ++i) {
            int x, y, k;
            cin >> x >> y >> k;
            // 自分が設置した爆弾はどうでもいい
            if (k == my_id) continue;
            field[y][x].type = k == -1 ? FieldState::BomSafe : FieldState::BomReady;
            field[y][x].val = k;
            if (k == -1) {
                bom_list.emplace_back(x, y);
                continue;
            }
            for (int dx = -1; dx <= 1; ++dx) {
                for (int dy = -1; dy <= 1; ++dy) {
                    int nx = dx + x, ny = dy + y;
                    if (nx < 0 || ny < 0 || nx >= size || ny >= size)
                        continue;
                    danger[ny][nx] = true;
                }
            }
        }
        // 何ターンか所持したら落とす
        if (bom_period > 0) --bom_period;
        if (bom_period == 0) {
            Actions::drop_bom();
            bom_period = -1;
        } else if (bom_period == -1 && field[py][px].type == FieldState::BomSafe) {
            bom_period = 3;
            Actions::pick_bom();
        } else if (bom_period > 0) {
            search_player();
        } else {
            search_bom();
        }
    }
    void move_right() {
        Actions::move(2);
    }
    // あるリソースから 他の全ての点への最短距離を計算
    vector<vector<int>> make_dist(DistType dtype) {
        vector<vector<int>> dist(size, vector<int>(size, INF));
        queue<pair<int, int>> que;
        const auto& resource = dtype == DistType::Bom ? bom_list : others_list;
        for (auto p : resource) {
            int x, y;
            tie(x, y) = p;
            dist[y][x] = 0;
            que.emplace(x, y);
        }
        while (!que.empty()) {
            int cx, cy;
            tie(cx, cy) = que.front(); que.pop();
            for (int i = 0; i < 4; ++i) {
                int nx = cx + DX[i], ny = cy + DY[i];
                if (nx < 0 || ny < 0 || nx >= size || ny >= size)
                    continue;
                if (dist[ny][nx] != INF) continue;
                if (field[ny][nx].type == FieldState::BomReady) continue;
                dist[ny][nx] = dist[cy][cx] + 1;
                que.emplace(nx, ny);
            }
        }
        return dist;
    }
    // 最も近い爆弾を探す
    void search_bom() {
        auto bom_dist = make_dist(DistType::Bom);
        for (int i = 0; i < 4; ++i) {
            int nx = px + DX[i];
            int ny = py + DY[i];
            if (nx < 0 || ny < 0 || nx >= size || ny >= size)
                continue;
            // 移動して最短距離が短くなるような点に移動する
            if (danger[ny][nx]) continue;
            if (bom_dist[ny][nx] < bom_dist[py][px]) {
                Actions::move(i);
                return;
            }
        }
        Actions::nop();
    }
    // 最も近いプレイヤーを探す
    void search_player() {
        auto p_dist = make_dist(DistType::Bom);
        for (int i = 0; i < 4; ++i) {
            int nx = px + DX[i];
            int ny = py + DY[i];
            if (nx < 0 || ny < 0 || nx >= size || ny >= size)
                continue;
            // 移動して最短距離が短くなるような点に移動する
            if (danger[ny][nx]) continue;
            if (p_dist[ny][nx] < p_dist[py][px]) {
                Actions::move(i);
                return;
            }
        }
        Actions::nop();
    }
};



int main() {
    int size;
    GameInfo game;
    while (cin >> size) {
        game.action(size);
    }
}

