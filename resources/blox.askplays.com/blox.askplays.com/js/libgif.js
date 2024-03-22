!function (e, t) {
    'function' == typeof define && define.amd ? define([], t) : 'object' == typeof exports ? module.exports = t() : e.SuperGif = t();
}(this, function () {
    function ne(e) {
        this.data = e;
        this.len = this.data.length;
        this.pos = 0;
        this.readByte = function () {
            if (this.pos >= this.data.length) {
                throw new Error('Attempted to read past end of stream.');
            }
            return e instanceof Uint8Array ? e[this.pos++] : 255 & e.charCodeAt(this.pos++);
        };
        this.readBytes = function (e) {
            for (var t = [], n = 0; n < e; n++) {
                t.push(this.readByte());
            }
            return t;
        };
        this.read = function (e) {
            for (var t = '', n = 0; n < e; n++) {
                t += String.fromCharCode(this.readByte());
            }
            return t;
        };
        this.readUnsigned = function () {
            var e = this.readBytes(2);
            return (e[1] << 8) + e[0];
        };
    }
    function re(i, o) {
        function s() {
            for (var e, t = ''; e = i.readByte(), t += i.read(e), 0 !== e;) {
                ;
            }
            return t;
        }
        o = o || {};
        var r = function (e) {
                for (var t = [], n = 0; n < e; n++) {
                    t.push(i.readBytes(3));
                }
                return t;
            }, t = function (e) {
                function t(e) {
                    var t;
                    i.readByte();
                    ;
                    ;
                    'NETSCAPE' === e.identifier ? (t = e, i.readByte(), t.unknown = i.readByte(), t.iterations = i.readUnsigned(), t.terminator = i.readByte(), o.app && o.app.NETSCAPE && o.app.NETSCAPE(t)) : ((t = e).appData = s(), o.app && o.app[t.identifier] && o.app[t.identifier](t));
                }
                var n, r;
                switch (e.label = i.readByte(), e.label) {
                case 249:
                    e.extType = 'gce', a = e, i.readByte(), r = c(i.readByte()), a.reserved = r.splice(0, 3), a.disposalMethod = d(r.splice(0, 3)), a.userInput = r.shift(), a.transparencyGiven = r.shift(), a.delayTime = i.readUnsigned(), a.transparencyIndex = i.readByte(), a.terminator = i.readByte(), o.gce && o.gce(a);
                    break;
                case 254:
                    e.extType = 'com', (r = e).comment = s(), o.com && o.com(r);
                    break;
                case 1:
                    ;
                    var a = e;
                    i.readByte(), a.ptHeader = i.readBytes(12), a.ptData = s(), o.pte && o.pte(a);
                    break;
                case 255:
                    e.extType = 'app', t(e);
                    break;
                default:
                    e.extType = 'unknown', (n = e).data = s(), o.unknown && o.unknown(n);
                }
            }, n = function (e) {
                function t(e, t) {
                    for (var n = new Array(e.length), r = e.length / t, a = [
                                0,
                                4,
                                2,
                                1
                            ], i = [
                                8,
                                8,
                                4,
                                2
                            ], o = 0, s = 0; s < 4; s++) {
                        for (var l = a[s]; l < r; l += i[s]) {
                            c = void 0;
                            var d = l, c = o;
                            c = e.slice(c * t, (c + 1) * t);
                            n.splice.apply(n, [
                                d * t,
                                t
                            ].concat(c));
                            o++;
                        }
                    }
                    return n;
                }
                ;
                ;
                ;
                ;
                var n = c(i.readByte()), n = (e.lctFlag = n.shift(), e.interlaced = n.shift(), e.sorted = n.shift(), e.reserved = n.splice(0, 2), e.lctSize = d(n.splice(0, 3)), e.lctFlag && (e.lct = r(1 << e.lctSize + 1)), e.lzwMinCodeSize = i.readByte(), s());
                ;
                e.interlaced && (e.pixels = t(e.pixels, e.width));
                o.img && o.img(e);
            }, a = function () {
                var e = {
                    identifier: i.read(8),
                    authCode: i.read(3),
                    extType: 'pte',
                    leftPos: i.readUnsigned(),
                    topPos: i.readUnsigned(),
                    width: i.readUnsigned(),
                    height: i.readUnsigned(),
                    pixels: h(e.lzwMinCodeSize, n),
                    width: i.readUnsigned(),
                    height: i.readUnsigned(),
                    gctFlag: l.shift(),
                    colorRes: d(l.splice(0, 3)),
                    sorted: l.shift(),
                    gctSize: d(l.splice(0, 3)),
                    bgColor: i.readByte(),
                    pixelAspectRatio: i.readByte()
                };
                switch (e.sentinel = i.readByte(), String.fromCharCode(e.sentinel)) {
                case '!':
                    e.type = 'ext', t(e);
                    break;
                case ',':
                    e.type = 'img', n(e);
                    break;
                case ';':
                    e.type = 'eof', o.eof && o.eof(e);
                    break;
                default:
                    throw new Error('Unknown block: 0x' + e.sentinel.toString(16));
                }
                'eof' !== e.type && setTimeout(a, 0);
            }, e = {};
        if (e.sig = i.read(3), e.ver = i.read(3), 'GIF' !== e.sig) {
            throw new Error('Not a GIF file.');
        }
        ;
        ;
        var l = c(i.readByte());
        ;
        ;
        ;
        ;
        ;
        ;
        e.gctFlag && (e.gct = r(1 << e.gctSize + 1));
        o.hdr && o.hdr(e);
        setTimeout(a, 0);
    }
    var d = function (e) {
            return e.reduce(function (e, t) {
                return 2 * e + t;
            }, 0);
        }, c = function (e) {
            for (var t = [], n = 7; 0 <= n; n--) {
                t.push(!!(e & 1 << n));
            }
            return t;
        }, h = function (e, r) {
            for (var t, n, a = 0, i = [], o = 1 << e, s = 1 + o, l = e + 1, d = [];;) {
                if (n = t, (t = function (e) {
                        for (var t = 0, n = 0; n < e; n++) {
                            r.charCodeAt(a >> 3) & 1 << (7 & a) && (t |= 1 << n);
                            a++;
                        }
                        return t;
                    }(l)) === o) {
                    c = void 0;
                    d = [];
                    l = e + 1;
                    for (var c = 0; c < o; c++) {
                        d[c] = [c];
                    }
                    d[o] = [];
                    d[s] = null;
                } else {
                    if (t === s) {
                        break;
                    }
                    if (t < d.length) {
                        n !== o && d.push(d[n].concat(d[t][0]));
                    } else {
                        if (t !== d.length) {
                            throw new Error('Invalid LZW code.');
                        }
                        d.push(d[n].concat(d[n][0]));
                    }
                    i.push.apply(i, d[t]);
                    d.length === 1 << l && l < 12 && l++;
                }
            }
            return i;
        };
    return function (e) {
        var t, n, a;
        for (t in e)
            o[t] = e[t];
        null && null && (o.is_vp = true);
        function N() {
            _ = g;
            w = g = f = p = null;
        }
        function D() {
            try {
                re(n, te);
            } catch (e) {
                r('parse');
            }
        }
        function F(e, t) {
            A.width = e * O();
            A.height = t * O();
            I.style.minWidth = e * O() + 'px';
            U.width = e;
            U.height = t;
            U.style.width = e + 'px';
            U.style.height = t + 'px';
            U.getContext('2d').setTransform(1, 0, 0, 1, 0, 0);
        }
        function r(e) {
            W = e;
            a = {
                width: B.width,
                height: B.height
            };
            b = [];
            E.fillStyle = 'black';
            E.fillRect(0, 0, null || a.width, null || a.height);
            E.strokeStyle = 'red';
            E.lineWidth = 3;
            E.moveTo(0, 0);
            E.lineTo(null || a.width, null || a.height);
            E.moveTo(0, null || a.height);
            E.lineTo(null || a.width, 0);
            E.stroke();
        }
        function M() {
            w && (b.push({
                data: w.getImageData(0, 0, a.width, a.height),
                delay: f
            }), T.push({
                x: 0,
                y: 0
            }));
        }
        var i, G, s, l, d, j, c, h, W = null, u = false, p = null, f = null, g = null, y = null, _ = null, w = null, v = null, m = true, x = false, b = [], T = [], B = o.gif, H = (void 0 === o.auto_play && (o.auto_play = !B.getAttribute('rel:auto_play') || '1' == B.getAttribute('rel:auto_play')), function hasOwnProperty() { [native code] }('on_end') ? o.on_end : null), q = function hasOwnProperty() { [native code] }('loop_delay') ? o.loop_delay : 0, L = function hasOwnProperty() { [native code] }('loop_mode') ? o.loop_mode : 'auto', C = !function hasOwnProperty() { [native code] }('draw_while_loading') || o.draw_while_loading, V = !!C && (!function hasOwnProperty() { [native code] }('show_progress_bar') || o.show_progress_bar), X = function hasOwnProperty() { [native code] }('progressbar_height') ? o.progressbar_height : 25, Z = function hasOwnProperty() { [native code] }('progressbar_background_color') ? o.progressbar_background_color : 'rgba(255,255,255,0.4)', J = function hasOwnProperty() { [native code] }('progressbar_foreground_color') ? o.progressbar_foreground_color : 'rgba(255,0,22,.8)', K = function (e, t, n) {
                var r, a, i;
                n && V && (n = X, o.is_vp ? i = x ? (a = (0 + null - n) / O(), n /= O(), r = 0 / O() + e / t * (null / O()), A.width / O()) : (a = 0 + null - n, r = 0 + e / t * null, A.width) : (a = (A.height - n) / (x ? O() : 1), r = e / t * A.width / (x ? O() : 1), i = A.width / (x ? O() : 1), n /= x ? O() : 1), E.fillStyle = Z, E.fillRect(r, a, i - r, n), E.fillStyle = J, E.fillRect(0, a, r, n));
            }, P = (l = -1, j = function () {
                return (l + 1 + b.length) % b.length;
            }, i = !(c = function (e) {
                l += e;
                S();
            }), {
                init: function () {
                    W || (null && null || E.scale(O(), O()), o.auto_play ? h() : (l = 0, S()));
                },
                step: h = function () {
                    i || setTimeout(s, 0);
                },
                play: function () {
                    m = true;
                    h();
                },
                pause: function () {
                    m = false;
                },
                playing: m,
                move_relative: c,
                current_frame: function () {
                    return l;
                },
                length: function () {
                    return b.length;
                },
                move_to: function (e) {
                    l = e;
                    S();
                }
            });
        function S() {
            var e;
            (l = parseInt(l, 10)) > b.length - 1 && (l = 0);
            e = T[l = l < 0 ? 0 : l];
            U.getContext('2d').putImageData(b[l].data, e.x, e.y);
            E.globalCompositeOperation = 'copy';
            E.drawImage(U, 0, 0);
        }
        function Q(e) {
            K(n.pos, n.data.length, e);
        }
        function Y() {
        }
        function k(t, n) {
            return function (e) {
                t(e);
                Q(n);
            };
        }
        function $() {
            var e = B.parentNode, t = document.createElement('div');
            A = document.createElement('canvas');
            E = A.getContext('2d');
            I = document.createElement('div');
            U = document.createElement('canvas');
            t.width = A.width = B.width;
            t.height = A.height = B.height;
            I.style.minWidth = B.width + 'px';
            t.className = 'jsgif';
            I.className = 'jsgif_toolbar';
            t.appendChild(A);
            t.appendChild(I);
            e && (e.insertBefore(t, B), e.removeChild(B));
            null && null && F(null, null);
            R = true;
        }
        function ee(e) {
            return !u && (z = e || false, u = true, b = [], N(), !(v = w = _ = y = null));
        }
        var A, E, I, U, te = {
                hdr: k(function (e) {
                    F((a = e).width, a.height);
                }),
                gce: k(function (e) {
                    M();
                    N();
                    p = e.transparencyGiven ? e.transparencyIndex : null;
                    f = e.delayTime;
                    g = e.disposalMethod;
                }),
                com: k(Y),
                app: { NETSCAPE: k(Y) },
                img: k(function (e) {
                    w = w || U.getContext('2d');
                    var t = b.length, n = e.lctFlag ? e.lct : a.gct, r = (0 < t && (3 === _ ? null !== y ? w.putImageData(b[y].data, 0, 0) : w.clearRect(v.leftPos, v.topPos, v.width, v.height) : y = t - 1, 2 === _ && w.clearRect(v.leftPos, v.topPos, v.width, v.height)), w.getImageData(e.leftPos, e.topPos, e.width, e.height));
                    e.pixels.forEach(function (e, t) {
                        e !== p && (r.data[4 * t + 0] = n[e][0], r.data[4 * t + 1] = n[e][1], r.data[4 * t + 2] = n[e][2], r.data[4 * t + 3] = 255);
                    });
                    w.putImageData(r, e.leftPos, e.topPos);
                    x || (E.scale(O(), O()), x = true);
                    C && (E.drawImage(U, 0, 0), C = o.auto_play);
                    v = e;
                }, !(d = 0)),
                eof: function (e) {
                    M();
                    Q(false);
                    null && null || (A.width = a.width * O(), A.height = a.height * O());
                    P.init();
                    u = false;
                    z && z(B);
                }
            }, O = function () {
                var e = o.max_width && a && a.width > o.max_width ? o.max_width / a.width : 1;
                return e;
            }, R = !(s = function () {
                var e;
                (i = m) && (c(1), e = (e = 10 * b[l].delay) || 100, 0 === j() ? (e += q, setTimeout(G, e)) : setTimeout(s, e));
            }), z = !(G = function () {
                null !== H && H(B);
                d++;
                false !== L || d < 0 ? s() : m = i = false;
            });
        return {
            play: P.play,
            pause: P.pause,
            move_relative: P.move_relative,
            move_to: P.move_to,
            get_frames: function () {
                return b;
            },
            get_playing: function () {
                return m;
            },
            get_canvas: function () {
                return A;
            },
            get_canvas_scale: function () {
                return O();
            },
            get_loading: function () {
                return u;
            },
            get_auto_play: function () {
                return o.auto_play;
            },
            get_length: function () {
                return P.length();
            },
            get_current_frame: function () {
                return P.current_frame();
            },
            load_url: function (e, t) {
                ee(t) && ((t = new XMLHttpRequest()).open('GET', e, true), 'overrideMimeType' in t ? t.overrideMimeType('text/plain; charset=x-user-defined') : 'responseType' in t ? t.responseType = 'arraybuffer' : t.setRequestHeader('Accept-Charset', 'x-user-defined'), t.onloadstart = function () {
                    R || $();
                }, t.onload = function (e) {
                    200 != this.status && r('xhr - response');
                    'response' in this || (this.response = new VBArray(this.responseText).toArray().map(String.fromCharCode).join(''));
                    var t = this.response;
                    0 < t.toString().indexOf('ArrayBuffer') && (t = new Uint8Array(t));
                    n = new ne(t);
                    setTimeout(D, 0);
                }, t.onprogress = function (e) {
                    e.lengthComputable && K(e.loaded, e.total, true);
                }, t.onerror = function () {
                    r('xhr');
                }, t.send());
            },
            load: function (e) {
                this.load_url(B.getAttribute('rel:animated_src') || B.src, e);
            },
            load_raw: function (e, t) {
                ee(t) && (R || $(), n = new ne(e), setTimeout(D, 0));
            },
            set_frame_offset: function (e, t) {
                T[e] ? (void 0 !== t.x && (T[e].x = t.x), void 0 !== t.y && (T[e].y = t.y)) : T[e] = t;
            }
        };
    };
});