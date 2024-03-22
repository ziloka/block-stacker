function encodeReplay() {
    var t = r.saveReplay()
    replay = {
      c: c,
      d: t,
    }
    addReplay(
      (replay = LZString.compressToEncodedURIComponent(JSON.stringify(replay)))
    )
  }
  function addReplay(r) {
    if (
      (document.getElementById('chat-box') &&
        ((!alive && 0 == mode) || websocket.readyState != websocket.OPEN)) ||
      2 == playMode ||
      desynced
    ) {
      let t = document.createElement('p'),
        e =
          ((t.classList = 'replay'),
          (t.innerHTML =
            '<a href="replay?r=' +
            r +
            '" target="_blank">replay<span class="icon"><img src="/res/icons/launch-icon.svg" alt="Open in new tab" width="17" height="17"/></span></a> '),
          document.createElement('input')),
        s =
          ((e.type = 'text'),
          e.setAttribute('value', r),
          (e.value = r),
          document.createElement('button')),
        i =
          ((s.innerText = 'copy'),
          s.addEventListener('click', copyText),
          t.appendChild(e),
          t.appendChild(s),
          document.getElementById('chat-box').appendChild(t),
          document.getElementById('chat-box'))
      i.scrollTop = i.scrollHeight
    }
  }
  class Replayer {
    constructor() {
      this.r = {}
      this.actions = []
    }
  }
  const r = new Replayer(),
    Action = Object.freeze({
      MOVE_LEFT: 0,
      MOVE_RIGHT: 1,
      DAS_LEFT: 2,
      DAS_RIGHT: 3,
      ROTATE_LEFT: 4,
      ROTATE_RIGHT: 5,
      ROTATE_180: 6,
      HARD_DROP: 7,
      SOFT_DROP_BEGIN_END: 8,
      GRAVITY_STEP: 9,
      HOLD_BLOCK: 10,
      GARBAGE_ADD: 11,
      SGARBAGE_ADD: 12,
      REDBAR_SET: 13,
      ARR_MOVE: 14,
      AUX: 15,
    }),
    Aux = Object.freeze({
      AFK: 0,
      BLOCK_SET: 1,
      MOVE_TO: 2,
      RANDOMIZER: 3,
      MATRIX_MOD: 4,
      WIDE_GARBAGE_ADD: 5,
      WIDE_GARBAGE_SET: 6,
    })
  function action(e, s, i) {
    if (1 === gamemode) {
      let t = {
        t: Math.max(0, counter),
        a: Action[e],
      }
      s && (t.d = s)
      i && (t.aux = Aux[i])
      r.actions.push(t)
      s
        ? sendActions.push({
            a: replayCodes[e],
            d: s,
          })
        : sendActions.push({ a: replayCodes[e] })
    }
  }
  function _base64ToArrayBuffer(t) {
    for (
      var e = window.atob(t), s = e.length, i = new Uint8Array(s), r = 0;
      r < s / 4;
      r++
    ) {
      for (var n = 0; n < 4; n++) {
        i[4 * r + n] = e.charCodeAt(4 * r + 3 - n)
      }
    }
    return i.buffer
  }
  function _arrayBufferToBase64(t) {
    for (var e = '', s = new Uint8Array(t), i = s.byteLength, r = 0; r < i; r++) {
      e += ' '
    }
    for (r = 0; r < i / 4; r++) {
      for (var n = 0; n < 4; n++) {
        e = e.replaceAt(4 * r + 3 - n, String.fromCharCode(s[4 * r + n]))
      }
    }
    return window.btoa(e)
  }
  function Mash() {
    function t(t) {
      for (var e = 0, s = (t = t.toString()).length; e < s; e++) {
        var i = 0.02519603282416938 * (r += t.charCodeAt(e))
        r = (i = (i - (r = i >>> 0)) * r) >>> 0
        r += 4294967296 * (i -= r)
      }
      return 2.3283064365386963e-10 * (r >>> 0)
    }
    var r = 4022871197
    return (t.version = 'Mash 0.9'), t
  }
  function ir(t) {
    var i,
      e = this,
      s =
        ((i = 4022871197),
        function (t) {
          t = t.toString()
          for (var e = 0; e < t.length; e++) {
            var s = 0.02519603282416938 * (i += t.charCodeAt(e))
            i = (s = (s -= i = s >>> 0) * i) >>> 0
            i += 4294967296 * (s -= i)
          }
          return 2.3283064365386963e-10 * (i >>> 0)
        })
    e.next = function () {
      var t = 2091639 * e.s0 + 2.3283064365386963e-10 * e.c
      return (e.s0 = e.s1), (e.s1 = e.s2), (e.s2 = t - (e.c = 0 | t))
    }
    e.c = 1
    e.s0 = s(' ')
    e.s1 = s(' ')
    e.s2 = s(' ')
    e.s0 -= s(t)
    e.s0 < 0 && (e.s0 += 1)
    e.s1 -= s(t)
    e.s1 < 0 && (e.s1 += 1)
    e.s2 -= s(t)
    e.s2 < 0 && (e.s2 += 1)
  }
  function s(t, e) {
    return (e.c = t.c), (e.s0 = t.s0), (e.s1 = t.s1), (e.s2 = t.s2), e
  }
  function alea(t, e) {
    var i = new ir(t),
      t = e && e.state,
      r = i.next
    return (
      (r.int32 = function () {
        return (4294967296 * i.next()) | 0
      }),
      (r.double = function () {
        return r() + 1.1102230246251565e-16 * ((2097152 * r()) | 0)
      }),
      (r.quick = r),
      t &&
        ('object' == typeof t && s(t, i),
        (r.state = function () {
          return s(i, {})
        })),
      r
    )
  }
  function Bag(t, s, i) {
    this.RNG = t
    this.usebag = []
    for (let e = 0; e < s; e++) {
      for (let t = 0; t < i; t++) {
        this.usebag.push(e)
      }
    }
    this.bag = this.usebag.slice(0)
  }
  String.prototype.replaceAt = function (t, e) {
    return this.substr(0, t) + e + this.substr(t + e.length)
  }
  Replayer.prototype.loadReplay = function (t) {
    for (
      var t = _base64ToArrayBuffer((this.r.d = t)),
        e =
          ((ab = t),
          (this.data = new Uint32Array(t)),
          (this.byte = 0),
          (this.bitpos = 0),
          this.data.length,
          0),
        s = 0;
      ;
  
    ) {
      var i = {
          t: 4094 * s + (e = r),
          a: this.pullBits(4),
        },
        r = this.pullBits(12)
      if (null === r || 4095 === r) {
        break
      }
      r < e && s++
      i.a === Action.GARBAGE_ADD && (i.d = [this.pullBits(5), this.pullBits(4)])
      i.a === Action.REDBAR_SET && (i.d = [this.pullBits(5)])
      i.a === Action.ARR_MOVE && (i.d = [this.pullBits(1)])
      i.a === Action.AUX &&
        ((i.aux = this.pullBits(4)),
        i.aux === Aux.AFK
          ? ((i.d = [this.pullBits(16)]),
            4094 <= (e += i.d[0] % 4094) && ((e -= 4094), s++),
            (s += (i.d[0] / 4094) >>> 0))
          : i.aux === Aux.BLOCK_SET
          ? (i.d = [this.pullBits(1), this.pullBits(4)])
          : i.aux === Aux.MOVE_TO
          ? (i.d = [this.pullBits(4) - 3, this.pullBits(5) - 12])
          : i.aux === Aux.RANDOMIZER
          ? (i.d = [this.pullBits(1), this.pullBits(5)])
          : i.aux === Aux.MATRIX_MOD
          ? (i.d = [this.pullBits(4), this.pullBits(5)])
          : (i.aux !== Aux.WIDE_GARBAGE_ADD && i.aux !== Aux.WIDE_GARBAGE_SET) ||
            (i.d = [
              this.pullBits(5),
              this.pullBits(4),
              this.pullBits(3),
              this.pullBits(1),
            ]))
      this.actions.push(i)
    }
    return (
      this.debug && console.log(JSON.stringify(this.actions)),
      (this.r.a = this.actions),
      this.actions
    )
  }
  Replayer.prototype.pullBits = function (t) {
    if (this.data.length === this.byte) {
      return null
    }
    var e = 32 - (this.bitpos + t)
    if (0 <= e) {
      var s = (this.data[this.byte] & (((1 << t) - 1) << e)) >>> e
    } else {
      var i = (this.data[this.byte] & ((1 << (t + e)) - 1)) << -e
      if (this.data.length === this.byte + 1 && 0 != e) {
        return null
      }
      s =
        i |
        ((this.data[this.byte + 1] & (((1 << -e) - 1) << (32 + e))) >>> (32 + e))
    }
    return (
      (this.bitpos = this.bitpos + t),
      32 <= this.bitpos && ((this.bitpos -= 32), this.byte++),
      s
    )
  }
  Replayer.prototype.saveReplay = function (e) {
    e = this.actions
    this.data = null
    this.data = []
    this.byte = 0
    for (let t = (this.bitpos = 0); t < e.length; t++) {
      this.putBits(e[t].t % 4094, 12)
      this.putBits(e[t].a, 4)
      e[t].a === Action.GARBAGE_ADD &&
        (this.putBits(e[t].d[0], 5), this.putBits(e[t].d[1], 4))
      e[t].a === Action.AUX &&
        (this.putBits(e[t].aux, 4),
        e[t].aux === Aux.BLOCK_SET &&
          (this.putBits(e[t].d[0], 1), this.putBits(e[t].d[1], 4)),
        e[t].aux === Aux.WIDE_GARBAGE_ADD &&
          (this.putBits(e[t].d[0], 5),
          this.putBits(e[t].d[1], 4),
          this.putBits(e[t].d[2], 3),
          this.putBits(e[t].d[3], 1)),
        e[t].aux === Aux.WIDE_GARBAGE_SET &&
          (this.putBits(e[t].d[0], 5),
          this.putBits(e[t].d[1], 4),
          this.putBits(e[t].d[2], 3),
          this.putBits(e[t].d[3], 1)))
    }
    return (
      0 != this.bitpos &&
        (this.data[this.byte] =
          (this.data[this.byte] << (32 - this.bitpos)) +
          ((1 << (32 - this.bitpos)) - 1)),
      (this.data = new Uint32Array(this.data)),
      (this.r.d = _arrayBufferToBase64(this.data.buffer)),
      this.r.d
    )
  }
  Replayer.prototype.putBits = function (t, e) {
    var s, i
    t &= (1 << e) - 1
    32 < this.bitpos + e
      ? ((s = 32 - this.bitpos),
        (i = this.bitpos + e - 32),
        (this.data[this.byte] = (this.data[this.byte] << s) + (t >>> i)),
        (this.data[this.byte + 1] = t & ((1 << i) - 1)))
      : (this.data[this.byte] = (this.data[this.byte] << e) + t)
    this.bitpos += e
    32 <= this.bitpos && ((this.bitpos -= 32), this.byte++)
  }
  Bag.prototype.getBlock = function () {
    var t = Math.floor(this.RNG() * this.bag.length),
      t = this.bag.splice(t, 1)[0]
    return 0 === this.bag.length && (this.bag = this.usebag.slice(0)), t
  }
  var LZString = (function () {
    function s(t, e) {
      if (!n[t]) {
        for (var s = 0; s < t.length; s++) {
          n[t][t.charAt(s)] = s
        }
      }
      return n[t][e]
    }
    var b = String.fromCharCode,
      i = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=',
      r = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-$',
      n = { t: {} },
      o = {
        compressToBase64: function (t) {
          if (null == t) {
            return ''
          }
          var e = o._compress(t, 6, function (t) {
            return i.charAt(t)
          })
          switch (e.length % 4) {
            default:
            case 0:
              return e
            case 1:
              return e + '==='
            case 2:
              return e + '=='
            case 3:
              return e + '='
          }
        },
        decompressFromBase64: function (e) {
          return null == e
            ? ''
            : '' == e
            ? null
            : o._decompress(e.length, 32, function (t) {
                return s(i, e.charAt(t))
              })
        },
        compressToUTF16: function (t) {
          return null == t
            ? ''
            : o._compress(t, 15, function (t) {
                return b(t + 32)
              }) + ' '
        },
        decompressFromUTF16: function (e) {
          return null == e
            ? ''
            : '' == e
            ? null
            : o._decompress(e.length, 16384, function (t) {
                return e.charCodeAt(t) - 32
              })
        },
        compressToUint8Array: function (t) {
          for (
            var e = o.compress(t),
              s = new Uint8Array(2 * e.length),
              i = 0,
              r = e.length;
            i < r;
            i++
          ) {
            var n = e.charCodeAt(i)
            s[2 * i] = n >>> 8
            s[2 * i + 1] = n % 256
          }
          return s
        },
        decompressFromUint8Array: function (t) {
          if (null == t) {
            return o.decompress(t)
          }
          for (var e = new Array(t.length / 2), s = 0, i = e.length; s < i; s++) {
            e[s] = 256 * t[2 * s] + t[2 * s + 1]
          }
          var r = []
          return (
            e.forEach(function (t) {
              r.push(b(t))
            }),
            o.decompress(r.join(''))
          )
        },
        compressToEncodedURIComponent: function (t) {
          return null == t
            ? ''
            : o._compress(t, 6, function (t) {
                return r.charAt(t)
              })
        },
        decompressFromEncodedURIComponent: function (e) {
          return null == e
            ? ''
            : '' == e
            ? null
            : ((e = e.replace(/ /g, '+')),
              o._decompress(e.length, 32, function (t) {
                return s(r, e.charAt(t))
              }))
        },
        compress: function (t) {
          return o._compress(t, 16, function (t) {
            return b(t)
          })
        },
        _compress: function (t, e, s) {
          if (null == t) {
            return ''
          }
          for (
            var i,
              r,
              n,
              o,
              a = { o: l++ },
              u = {},
              h = '',
              p = 2,
              l = 3,
              c = 2,
              d = [],
              f = 0,
              A = 0,
              y = 0;
            y < t.length;
            y += 1
          ) {
            if (
              ((n = t.charAt(y)),
              Object.prototype.hasOwnProperty.call(a, n) ||
                ((a[n] = l++), (u[n] = true)),
              (o = h + n),
              Object.prototype.hasOwnProperty.call(a, o))
            ) {
              h = o
            } else {
              if (Object.prototype.hasOwnProperty.call(u, h)) {
                if (h.charCodeAt(0) < 256) {
                  for (i = 0; i < c; i++) {
                    f <<= 1
                    A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                  }
                  for (r = h.charCodeAt(0), i = 0; i < 8; i++) {
                    f = (f << 1) | (1 & r)
                    A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                    r >>= 1
                  }
                } else {
                  for (r = 1, i = 0; i < c; i++) {
                    f = (f << 1) | r
                    A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                    r = 0
                  }
                  for (r = h.charCodeAt(0), i = 0; i < 16; i++) {
                    f = (f << 1) | (1 & r)
                    A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                    r >>= 1
                  }
                }
                0 == --p && ((p = Math.pow(2, c)), c++)
                delete u[h]
              } else {
                for (r = a[h], i = 0; i < c; i++) {
                  f = (f << 1) | (1 & r)
                  A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                  r >>= 1
                }
              }
              0 == --p && ((p = Math.pow(2, c)), c++)
              h = String(n)
            }
          }
          if ('' !== h) {
            if (Object.prototype.hasOwnProperty.call(u, h)) {
              if (h.charCodeAt(0) < 256) {
                for (i = 0; i < c; i++) {
                  f <<= 1
                  A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                }
                for (r = h.charCodeAt(0), i = 0; i < 8; i++) {
                  f = (f << 1) | (1 & r)
                  A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                  r >>= 1
                }
              } else {
                for (r = 1, i = 0; i < c; i++) {
                  f = (f << 1) | r
                  A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                  r = 0
                }
                for (r = h.charCodeAt(0), i = 0; i < 16; i++) {
                  f = (f << 1) | (1 & r)
                  A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                  r >>= 1
                }
              }
              0 == --p && ((p = Math.pow(2, c)), c++)
              delete u[h]
            } else {
              for (r = a[h], i = 0; i < c; i++) {
                f = (f << 1) | (1 & r)
                A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
                r >>= 1
              }
            }
            0 == --p && ((p = Math.pow(2, c)), c++)
          }
          for (r = 2, i = 0; i < c; i++) {
            f = (f << 1) | (1 & r)
            A == e - 1 ? ((A = 0), d.push(s(f)), (f = 0)) : A++
            r >>= 1
          }
          for (;;) {
            if (((f <<= 1), A == e - 1)) {
              d.push(s(f))
              break
            }
            A++
          }
          return d.join('')
        },
        decompress: function (e) {
          return null == e
            ? ''
            : '' == e
            ? null
            : o._decompress(e.length, 32768, function (t) {
                return e.charCodeAt(t)
              })
        },
        _decompress: function (t, e, s) {
          for (
            var i,
              r,
              n,
              o,
              a,
              u,
              h = [],
              p = 4,
              l = 4,
              c = 3,
              d = '',
              f = [],
              A = {
                val: s(0),
                position: e,
                index: 1,
              },
              y = 0;
            y < 3;
            y += 1
          ) {
            h[y] = y
          }
          for (r = 0, o = Math.pow(2, 2), a = 1; a != o; ) {
            n = A.val & A.position
            A.position >>= 1
            0 == A.position && ((A.position = e), (A.val = s(A.index++)))
            r |= (0 < n ? 1 : 0) * a
            a <<= 1
          }
          switch (r) {
            case 0:
              for (r = 0, o = Math.pow(2, 8), a = 1; a != o; ) {
                n = A.val & A.position
                A.position >>= 1
                0 == A.position && ((A.position = e), (A.val = s(A.index++)))
                r |= (0 < n ? 1 : 0) * a
                a <<= 1
              }
              u = b(r)
              break
            case 1:
              for (r = 0, o = Math.pow(2, 16), a = 1; a != o; ) {
                n = A.val & A.position
                A.position >>= 1
                0 == A.position && ((A.position = e), (A.val = s(A.index++)))
                r |= (0 < n ? 1 : 0) * a
                a <<= 1
              }
              u = b(r)
              break
            case 2:
              return ''
          }
          for (i = h[3] = u, f.push(u); ; ) {
            if (A.index > t) {
              return ''
            }
            for (r = 0, o = Math.pow(2, c), a = 1; a != o; ) {
              n = A.val & A.position
              A.position >>= 1
              0 == A.position && ((A.position = e), (A.val = s(A.index++)))
              r |= (0 < n ? 1 : 0) * a
              a <<= 1
            }
            switch ((u = r)) {
              case 0:
                for (r = 0, o = Math.pow(2, 8), a = 1; a != o; ) {
                  n = A.val & A.position
                  A.position >>= 1
                  0 == A.position && ((A.position = e), (A.val = s(A.index++)))
                  r |= (0 < n ? 1 : 0) * a
                  a <<= 1
                }
                ;(h[l++] = b(r)), (u = l - 1), p--
                break
              case 1:
                for (r = 0, o = Math.pow(2, 16), a = 1; a != o; ) {
                  n = A.val & A.position
                  A.position >>= 1
                  0 == A.position && ((A.position = e), (A.val = s(A.index++)))
                  r |= (0 < n ? 1 : 0) * a
                  a <<= 1
                }
                ;(h[l++] = b(r)), (u = l - 1), p--
                break
              case 2:
                return f.join('')
            }
            if ((0 == p && ((p = Math.pow(2, c)), c++), h[u])) {
              d = h[u]
            } else {
              if (u !== l) {
                return null
              }
              d = i + i.charAt(0)
            }
            f.push(d)
            h[l++] = i + d.charAt(0)
            i = d
            0 == --p && ((p = Math.pow(2, c)), c++)
          }
        },
      }
    return o
  })()
  'function' == typeof define && define.amd
    ? define(function () {
        return LZString
      })
    : 'undefined' != typeof module &&
      null != module &&
      (module.exports = LZString)
  