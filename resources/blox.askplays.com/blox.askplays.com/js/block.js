let blockId = 0
class Block {
  constructor(t, s, i) {
    this.x = t
    this.y = s
    this.color = i
    this.id = blockId
  }
  update() {}
  show() {
    this.sprite(
      prefix,
      this.color,
      bx + 16 * this.x * 2 * ratio,
      16 * this.y * 2 * ratio,
      0,
      32 * ratio,
      32 * ratio
    )
  }
  sprite(s, i, h = 0, p = 0, t, A = 16, e = 16) {
    if (presetSkins.includes(s)) {
      settings.webgl
        ? drawImage(images[s + cols[i]], h, p, A, e)
        : ((ctx.fillStyle = hexes[s][i]), ctx.fillRect(h, p, A, e))
    } else {
      if (
        'undefined' != typeof settings &&
        settings &&
        settings.skinConnected &&
        'custom' == s
      ) {
        var o = sprites[s],
          r =
            (o || console.log(s),
            (board[-1] = []),
            (board[-2] = []),
            (board[-3] = []),
            (board[bW] = []),
            22)
        let t = 4 * (cols[i] - 2)
        3 < cols[i] - 2 && (t += 116)
        0 == cols[i] && (t += 152)
        var l = h / (32 * ratio),
          c = p / (32 * ratio)
        this.pAt(l - 1, c) &&
        this.pAt(1 + l, c) &&
        this.pAt(l, c - 1) &&
        this.pAt(l, 1 + c)
          ? (t += 24)
          : this.pAt(l - 1, c) &&
            this.pAt(1 + l, c) &&
            this.pAt(l, c - 1) &&
            !this.pAt(l, 1 + c) &&
            this.pAt(l - 1, c - 1) &&
            this.pAt(1 + l, c - 1)
          ? (t += 46)
          : !this.pAt(l - 1, c) &&
            this.pAt(1 + l, c) &&
            this.pAt(l, c - 1) &&
            this.pAt(l, 1 + c) &&
            this.pAt(1 + l, c - 1) &&
            this.pAt(1 + l, 1 + c)
          ? (t += 23)
          : this.pAt(l - 1, c) &&
            this.pAt(1 + l, c) &&
            !this.pAt(l, c - 1) &&
            this.pAt(l, 1 + c) &&
            this.pAt(l - 1, 1 + c) &&
            this.pAt(1 + l, 1 + c)
          ? (t += 2)
          : this.pAt(l - 1, c) &&
            !this.pAt(1 + l, c) &&
            this.pAt(l, c - 1) &&
            this.pAt(l, 1 + c) &&
            this.pAt(l - 1, c - 1) &&
            this.pAt(l - 1, 1 + c)
          ? (t += 25)
          : !this.pAt(l - 1, c) &&
            this.pAt(1 + l, c) &&
            !this.pAt(l, c - 1) &&
            this.pAt(l, 1 + c) &&
            this.pAt(1 + l, 1 + c)
          ? (t += 1)
          : this.pAt(l - 1, c) &&
            !this.pAt(1 + l, c) &&
            !this.pAt(l, c - 1) &&
            this.pAt(l, 1 + c) &&
            this.pAt(l - 1, 1 + c)
          ? (t += 3)
          : !this.pAt(l - 1, c) &&
            this.pAt(1 + l, c) &&
            this.pAt(l, c - 1) &&
            !this.pAt(l, 1 + c) &&
            this.pAt(1 + l, c - 1)
          ? (t += 45)
          : this.pAt(l - 1, c) &&
            !this.pAt(1 + l, c) &&
            this.pAt(l, c - 1) &&
            !this.pAt(l, 1 + c) &&
            this.pAt(l - 1, c - 1)
          ? (t += 47)
          : this.pAt(l - 1, c) ||
            this.pAt(1 + l, c) ||
            this.pAt(l, c - 1) ||
            !this.pAt(l, 1 + c)
          ? !this.pAt(l - 1, c) &&
            !this.pAt(1 + l, c) &&
            this.pAt(l, c - 1) &&
            this.pAt(l, 1 + c)
            ? (t += r)
            : this.pAt(l - 1, c) ||
              this.pAt(1 + l, c) ||
              !this.pAt(l, c - 1) ||
              this.pAt(l, 1 + c)
            ? this.pAt(l - 1, c) ||
              !this.pAt(1 + l, c) ||
              this.pAt(l, c - 1) ||
              this.pAt(l, 1 + c)
              ? this.pAt(l - 1, c) &&
                this.pAt(1 + l, c) &&
                !this.pAt(l, c - 1) &&
                !this.pAt(l, 1 + c)
                ? (t += 68)
                : !this.pAt(l - 1, c) ||
                  this.pAt(1 + l, c) ||
                  this.pAt(l, c - 1) ||
                  this.pAt(l, 1 + c)
                ? this.pAt(l - 1, c) ||
                  this.pAt(1 + l, c) ||
                  this.pAt(l, c - 1) ||
                  this.pAt(l, 1 + c)
                  ? !this.pAt(l - 1, c) &&
                    this.pAt(1 + l, c) &&
                    !this.pAt(l, c - 1) &&
                    this.pAt(l, 1 + c)
                    ? (t += 88)
                    : this.pAt(l - 1, c) &&
                      !this.pAt(1 + l, c) &&
                      !this.pAt(l, c - 1) &&
                      this.pAt(l, 1 + c)
                    ? (t += 89)
                    : !this.pAt(l - 1, c) &&
                      this.pAt(1 + l, c) &&
                      this.pAt(l, c - 1) &&
                      !this.pAt(l, 1 + c)
                    ? (t += 110)
                    : this.pAt(l - 1, c) &&
                      !this.pAt(1 + l, c) &&
                      this.pAt(l, c - 1) &&
                      !this.pAt(l, 1 + c)
                    ? (t += 111)
                    : this.pAt(l - 1, c) &&
                      this.pAt(1 + l, c) &&
                      this.pAt(l, c - 1) &&
                      !this.pAt(l, 1 + c)
                    ? (t += 90)
                    : !this.pAt(l - 1, c) &&
                      this.pAt(1 + l, c) &&
                      this.pAt(l, c - 1) &&
                      this.pAt(l, 1 + c)
                    ? (t += 91)
                    : this.pAt(l - 1, c) &&
                      this.pAt(1 + l, c) &&
                      !this.pAt(l, c - 1) &&
                      this.pAt(l, 1 + c)
                    ? (t += 112)
                    : this.pAt(l - 1, c) &&
                      !this.pAt(1 + l, c) &&
                      this.pAt(l, c - 1) &&
                      this.pAt(l, 1 + c) &&
                      (t += 113)
                  : (t += 66)
                : (t += 69)
              : (t += 67)
            : (t += 44)
          : (t += 0)
        r = o.sprites[t % o.sprites.length]
        try {
          ctx.drawImage(r, 0, 0, o.w, o.h, h, p, A, e)
        } catch (t) {}
      } else {
        l = sprites[s]
        c = l.sprites[cols[i] % l.sprites.length]
        ctx.drawImage(c, 0, 0, l.w, l.h, h, p, A, e)
      }
    }
  }
  pAt(t, s) {
    return (
      board[t][s] &&
      board[t][s].color == this.color &&
      board[t][s].id == this.id
    )
  }
}
