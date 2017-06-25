// 一番近い石油を掘りに行くAI
#include <iostream>
#include <vector>
#include <string>
#include <queue>
#include <tuple>
using namespace std;

namespace Actions {
void move(int id) {
    cout << 1 << endl;
    cout << id << endl;
    cerr << 1 << ' ' << id << ">_<" << endl;
}
void nop() {
    cout << 0 << endl;
}
void pick_bom() {
    cout << 2 << endl;
}
void drop_bom() {
    cout << 3 << endl;
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
struct GameInfo {
    vector<vector<FieldVal>> field;
    int size;
    int galon;
    int x;
    int y;
    int player_num;
    explicit GameInfo(): field(), size(0), galon(0) {}
    void action(int s) {
        size = s;
        field.assign(s, vector<FieldVal>(s));
        int oil_num, bom_num;
        cin >> oil_num;
        for (int i = 0; i < oil_num; ++i) {
            int x, y, g;
            cin >> x >> y >> g;
            field[y][x].type = FieldState::Galon;
            field[y][x].val = g;
        }
        cin >> bom_num;
        for (int i = 0; i < bom_num; ++i) {
            int x, y, k;
            cin >> x >> y >> k;
        }
        int my_id;
        cin >> player_num >> my_id;
        for (int i = 0; i < player_num; ++i) {
            int a, b, c;
            cin >> a >> b >> c;
            if (i == my_id) {
                x = a;
                y = b;
            }
        }
        move_greedy();
        //move_right();
    }
    void move_right() {
        Actions::move(2);
    }
    // (aim_x, aim_y) から 他の全ての点への最短距離を計算
    vector<vector<int>> make_dist(int aim_x, int aim_y) {
        vector<vector<int>> dist(size, vector<int>(size, INF));
        queue<pair<int, int>> que;
        dist[aim_y][aim_x] = 0;
        que.emplace(aim_x, aim_y);
        while (!que.empty()) {
            int cx, cy;
            tie(cx, cy) = que.front(); que.pop();
            for (int i = 0; i < 4; ++i) {
                int nx = cx + DX[i], ny = cy + DY[i];
                if (nx < 0 || ny < 0 || nx >= size || ny >= size)
                    continue;
                if (dist[ny][nx] != INF) continue;
                dist[ny][nx] = dist[cy][cx] + 1;
                que.emplace(nx, ny);
            }
        }
        return dist;
    }
    // 幅優先探索で最も近い石油を探す
    // グリッドグラフは二点間の距離が必ず1なので 経由したノード数+1 = 最短距離が成り立つ
    void move_greedy() {
        auto dist_from_now = make_dist(x, y);
        int mi = INF;
        int aim_x = 0;
        int aim_y = 0;
        for (int i = 0; i < size; ++i) {
            for (int j = 0; j < size; ++j) {
                if (field[i][j].type == FieldState::Galon) {
                    if (dist_from_now[i][j] < mi) {
                        mi = dist_from_now[i][j];
                        aim_x = j;
                        aim_y = i;
                    }
                }
            }
        }
        auto dist = make_dist(aim_x, aim_y);
        for (int i = 0; i < 4; ++i) {
            int nx = x + DX[i];
            int ny = y + DY[i];
            if (nx < 0 || ny < 0 || nx >= size || ny >= size)
                continue;
            // 移動して最短距離が短くなるような点に移動する
            if (dist[ny][nx] < dist[y][x]) {
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

