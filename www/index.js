"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const pkg_1 = require("../pkg");
function get_hints(h, w, board) {
    const hints = new Array();
    for (let y = 0; y < h; y++) {
        const line = Array();
        for (let x = 0; x < w;) {
            let nx = x;
            while (nx < w && board[y * w + x] == board[y * w + nx])
                nx++;
            if (board[y * w + x])
                line.push(nx - x);
            x = nx;
        }
        hints.push(line.length);
        hints.push.apply(hints, line);
    }
    for (let x = 0; x < w; x++) {
        const line = Array();
        for (let y = 0; y < h;) {
            let ny = y;
            while (ny < h && board[y * w + x] == board[ny * w + x])
                ny++;
            if (board[y * w + x])
                line.push(ny - y);
            y = ny;
        }
        hints.push(line.length);
        hints.push.apply(hints, line);
    }
    return new Int32Array(hints);
}
let N = 5;
let ans = (0, pkg_1.gen)(N, N, BigInt(0));
let hints = get_hints(N, N, ans);
let board = new Int32Array;
let gamingBoardSvgs;
let cursor = { x: 0, y: 0 };
let pre = { x: -1, y: -1, ctrl: false, enter: false, undo: false };
let pressEnter = false;
let undoHistory = new Array();
let redoHistory = new Array();
let cleared = false;
let gameover = false;
let val = null;
function isCorrect(board, ans) {
    for (let i = 0; i < N * N; i++) {
        if (board[i] !== ans[i])
            return false;
    }
    return true;
}
const KEY_LEFT = 'a';
const KEY_RIGHT = 'd';
const KEY_UP = 'w';
const KEY_DOWN = 's';
const KEY_UNDO = 'z';
const KEY_REDO = 'y';
const FALSE = 0;
const TRUE = 1;
const NONE = 2;
document.onkeydown = function (ev) {
    if (gameover)
        return;
    if (!cleared && ev.key == KEY_LEFT) {
        cursor.x = (cursor.x - 1 + N) % N;
        document.getElementById("sushi").innerHTML = (0, pkg_1.vis_cursor)(N, N, cursor.y, cursor.x);
        pre.enter = false;
    }
    if (!cleared && ev.key == KEY_RIGHT) {
        cursor.x = (cursor.x + 1 + N) % N;
        document.getElementById("sushi").innerHTML = (0, pkg_1.vis_cursor)(N, N, cursor.y, cursor.x);
        pre.enter = false;
    }
    if (!cleared && ev.key == KEY_UP) {
        cursor.y = (cursor.y - 1 + N) % N;
        document.getElementById("sushi").innerHTML = (0, pkg_1.vis_cursor)(N, N, cursor.y, cursor.x);
        pre.enter = false;
    }
    if (!cleared && ev.key == KEY_DOWN) {
        cursor.y = (cursor.y + 1 + N) % N;
        document.getElementById("sushi").innerHTML = (0, pkg_1.vis_cursor)(N, N, cursor.y, cursor.x);
        pre.enter = false;
    }
    if (!cleared && ev.key == 'Enter') {
        pressEnter = true;
    }
    if (!cleared && pressEnter) {
        if (!pre.enter || pre.ctrl !== ev.ctrlKey) {
            if (!pre.undo)
                redoHistory = [];
            undoHistory.push([board, { x: cursor.x, y: cursor.y }]);
            pre.x = cursor.x, pre.y = cursor.y, pre.ctrl = ev.ctrlKey, pre.undo = false;
            if (val === null)
                val = !ev.ctrlKey && board[cursor.y * N + cursor.x] !== TRUE ? true
                    : ev.ctrlKey && board[cursor.y * N + cursor.x] !== FALSE ? false
                        : undefined;
            board = (0, pkg_1.set)(cursor.y, cursor.x, val, N, N, board, hints);
            document.getElementById("gyouza").innerHTML = (0, pkg_1.vis_board)(N, N, board, hints);
            pressEnter = true;
            pre.enter = true;
            if (isHardMode.checked && val !== undefined && ans[cursor.y * N + cursor.x] !== +val) {
                gameover = true;
                showGameover();
                return;
            }
        }
    }
    if (ev.key == KEY_UNDO && ev.ctrlKey) {
        if (undoHistory.length == 0)
            return;
        if (!pre.undo)
            redoHistory = [];
        redoHistory.push([board, cursor]);
        [board, cursor] = undoHistory.pop();
        document.getElementById("gyouza").innerHTML = (0, pkg_1.vis_board)(N, N, board, hints);
        document.getElementById("sushi").innerHTML = (0, pkg_1.vis_cursor)(N, N, cursor.y, cursor.x);
        pre.x = cursor.x, pre.y = cursor.y, pre.ctrl = false, pre.undo = true;
    }
    if (ev.key == KEY_REDO && ev.ctrlKey) {
        if (redoHistory.length == 0)
            return;
        undoHistory.push([board, cursor]);
        [board, cursor] = redoHistory.pop();
        document.getElementById("gyouza").innerHTML = (0, pkg_1.vis_board)(N, N, board, hints);
        document.getElementById("sushi").innerHTML = (0, pkg_1.vis_cursor)(N, N, cursor.y, cursor.x);
        pre.x = cursor.x, pre.y = cursor.y, pre.ctrl = false, pre.undo = false;
    }
    const correct = isCorrect(board, ans);
    if (!cleared && correct) {
        showFoot();
        cleared = true;
        if (isGamingMode.checked) {
            gamingBoardSvgs = (0, pkg_1.vis_gaming_boards)(N, N, board, hints).split("$");
            let t = 0;
            function drawGaming() {
                if (cleared) {
                    document.getElementById("gyouza").innerHTML = gamingBoardSvgs[t];
                    t += 1;
                    if (t >= 12)
                        t -= 12;
                    requestAnimationFrame(drawGaming);
                }
            }
            drawGaming();
        }
    }
    else if (cleared && !correct) {
        hideAll();
        cleared = false;
    }
};
document.onkeyup = function (ev) {
    if (ev.key == 'Enter') {
        pressEnter = false;
        pre.x = cursor.x, pre.y = cursor.y, pre.ctrl = false, pre.enter = false;
        val = null;
    }
};
const seedInput = document.getElementById("seed");
const sizeSelect = document.getElementById("size");
const copyButton = document.getElementById("copy");
const isGamingMode = document.getElementById("gaming");
const isHardMode = document.getElementById("hard");
const nextButtton = document.getElementById("next");
const savePngButton = document.getElementById("save_png");
const saveGifButton = document.getElementById("save_gif");
const shareButton = document.getElementById("share");
const nextHardButton = document.getElementById("next_hard");
const retryButton = document.getElementById("retry");
seedInput.onchange = function () {
    const seed = seedInput.value;
    const url = new URL(location.toString());
    url.searchParams.set('seed', seed);
    location.href = url.toString();
};
sizeSelect.onchange = function () {
    N = parseInt(sizeSelect.options[sizeSelect.selectedIndex].value);
    const url = new URL(location.toString());
    url.searchParams.set('size', `${N}`);
    location.href = url.toString();
};
copyButton.onclick = function () {
    const url = new URL(location.toString());
    const seed = seedInput.value;
    url.searchParams.set('size', `${N}`);
    url.searchParams.set('seed', `${seed}`);
    navigator.clipboard.writeText(url.toString()).then(function () {
        /* clipboard successfully set */
    }, function () {
        /* clipboard write failed */
    });
};
isGamingMode.onclick = function () {
    if (isGamingMode.checked) {
        sessionStorage.setItem('gaming', 'true');
    }
    else {
        sessionStorage.setItem('gaming', 'false');
    }
};
isHardMode.onclick = function () {
    if (isHardMode.checked) {
        sessionStorage.setItem('hard', 'true');
    }
    else {
        sessionStorage.setItem('hard', 'false');
    }
};
function newGame(seed) {
    ans = (0, pkg_1.gen)(N, N, seed);
    hints = get_hints(N, N, ans);
    board = new Int32Array(N * N).fill(2);
    cursor = { x: 0, y: 0 };
    pre = { x: 0, y: 0, ctrl: false, enter: false, undo: false };
    pressEnter = false;
    cleared = false;
    gameover = false;
    undoHistory = new Array([board, { x: 0, y: 0 }]);
    redoHistory = new Array();
    document.getElementById("gyouza").innerHTML = (0, pkg_1.vis_board)(N, N, board, hints);
    document.getElementById("sushi").innerHTML = (0, pkg_1.vis_cursor)(N, N, 0, 0);
}
function hideAll() {
    document.getElementById("foot").style.visibility = 'hidden';
    document.getElementById("foot").style.position = 'relative';
    document.getElementById("foot").style.top = `${N * 24 + 160}px`;
    document.getElementById("commands").style.position = 'relative';
    document.getElementById("commands").style.top = `${N * 24 + 230}px`;
    document.getElementById("gameover").style.visibility = 'hidden';
    document.getElementById("gameover").style.position = 'relative';
    document.getElementById("gameover").style.top = `${N * 24 + 70}px`;
}
function showFoot() {
    document.getElementById("foot").style.visibility = 'visible';
    document.getElementById("commands").style.top = `${N * 24 + 340}px`;
}
function showGameover() {
    document.getElementById("gameover").style.visibility = 'visible';
    document.getElementById("commands").style.top = `${N * 24 + 270}px`;
}
function load() {
    const url = new URL(location.toString());
    N = parseInt(url.searchParams.get('size') || "10");
    const seed = url.searchParams.get('seed') || (0, pkg_1.gen_seed)();
    if (!url.searchParams.has('size') || !url.searchParams.has('seed')) {
        url.searchParams.set('size', N.toString());
        url.searchParams.set('seed', seed);
        location.href = url.toString();
    }
    seedInput.value = seed;
    sizeSelect.options[N / 5 - 1].selected = true;
    isGamingMode.checked = sessionStorage.getItem('gaming') === 'true';
    isHardMode.checked = sessionStorage.getItem('hard') === 'true';
    hideAll();
    newGame(BigInt(seed));
}
load();
window.onload = load;
nextButtton.onclick = function () {
    const seed = (0, pkg_1.gen_seed)();
    const url = new URL(location.toString());
    url.searchParams.set('seed', seed);
    location.href = url.toString();
};
nextHardButton.onclick = function () {
    const seed = (0, pkg_1.gen_seed)();
    const url = new URL(location.toString());
    url.searchParams.set('seed', seed);
    location.href = url.toString();
};
retryButton.onclick = function () {
    const seed = seedInput.value;
    hideAll();
    newGame(BigInt(seed));
};
savePngButton.onclick = function () {
    const svgData = (0, pkg_1.vis_grid)(N, N, 15, board);
    const svg = new DOMParser().parseFromString(svgData, "image/svg+xml").getElementById("vis");
    const canvas = document.createElement("canvas");
    canvas.width = Number(svg === null || svg === void 0 ? void 0 : svg.getAttribute("width"));
    canvas.height = Number(svg === null || svg === void 0 ? void 0 : svg.getAttribute("height"));
    const ctx = canvas.getContext("2d");
    const image = new Image;
    image.onload = function () {
        ctx.drawImage(image, 0, 0);
        const a = document.createElement("a");
        const seed = seedInput.value;
        a.href = canvas.toDataURL("image/png");
        a.download = `${seed}.png`;
        a.click();
    };
    image.src = "data:image/svg+xml;charset=utf-8;base64," + btoa(unescape(encodeURIComponent(svgData)));
};
saveGifButton.onclick = function () {
    const boards = Int32Array.from(undoHistory.map((his) => Array.from(his[0])).flat());
    const d = 100 / N;
    const svgDatas = (0, pkg_1.vis_gif)(N, N, d, boards, undoHistory.length).split("$");
    saveGifButton.disabled = true;
    saveGifButton.value = "Generating GIF...";
    const _image = new Image;
    _image.onload = function () {
        const width = d * N;
        const height = d * N;
        const GIFEncoder = require('gifencoder');
        const encoder = new GIFEncoder(width, height);
        encoder.setRepeat(-1); // 0 for repeat, -1 for no-repeat
        encoder.setDelay(200); // frame delay in ms
        encoder.setQuality(5); // image quality. 10 is default.
        encoder.start();
        rec(0);
        function rec(t) {
            if (t == undoHistory.length) {
                encoder.finish();
                const a = document.createElement("a");
                const blob = new Blob([encoder.out.getData()], { type: 'image/gif' });
                a.href = URL.createObjectURL(blob);
                const seed = seedInput.value;
                a.download = `${seed}.gif`;
                a.click();
                saveGifButton.disabled = false;
                saveGifButton.value = "Save as Animation GIF";
                return;
            }
            saveGifButton.value = `Generating GIF... ${Math.floor(100 * t / undoHistory.length)}%`;
            const canvas = document.createElement("canvas");
            canvas.width = width;
            canvas.height = height;
            const ctx = canvas.getContext("2d");
            const image = new Image;
            image.onload = function () {
                ctx.drawImage(image, 0, 0);
                encoder.addFrame(ctx);
                rec(t + 1);
            };
            image.src = "data:image/svg+xml;charset=utf-8;base64," + btoa(unescape(encodeURIComponent(svgDatas[t])));
        }
    };
    _image.src = "data:image/svg+xml;charset=utf-8;base64," + btoa(unescape(encodeURIComponent(svgDatas[0])));
};
shareButton.onclick = function () {
    const seed = seedInput.value;
    const text = `無限イラロジ ${N}x${N} の Seed = ${seed} をクリア！🥟🍣`;
    const url = new URL(location.toString());
    url.searchParams.set('size', `${N}`);
    url.searchParams.set('seed', `${seed}`);
    const hashtag = 'mugen_illu_logi';
    const link = `https://twitter.com/intent/tweet?hashtags=${hashtag}&text=${text}%0D%0A&url=${url.toString().replace('&', '%26')}`;
    window.open(link);
};
