const cols = {
    outline: 0,
    perma: 1,
    garbage: 0,
    red: 2,
    orange: 3,
    yellow: 4,
    green: 5,
    teal: 6,
    blue: 7,
    purple: 8,
  },
  presetSkins = ['default', 'jstris', 'four', 'fourTop', 'preview'],
  hexes = {
    default: {
      teal: '#42AFE1',
      yellow: '#F6D03C',
      purple: '#9739A2',
      orange: '#F38927',
      blue: '#1165B5',
      green: '#51B84D',
      red: '#EB4F65',
      garbage: '#6A6A6A',
    },
    jstris: {
      teal: '#0F9BD7',
      yellow: '#E39F02',
      purple: '#AF298A',
      orange: '#E35B02',
      blue: '#2141C6',
      green: '#59B101',
      red: '#D70F37',
      garbage: '#6A6A6A',
    },
    four: {
      teal: '#42AFE1',
      yellow: '#F6D03C',
      purple: '#9739A2',
      orange: '#F38927',
      blue: '#1165B5',
      green: '#51B84D',
      red: '#EB4F65',
      garbage: '#6A6A6A',
    },
    fourTop: {
      teal: '#6CEAFF',
      yellow: '#FFFF7F',
      purple: '#D958E9',
      orange: '#FFBA59',
      blue: '#339BFF',
      green: '#84F880',
      red: '#FF7F79',
      garbage: '#DDDDDD',
    },
    preview: {
      teal: '#0F9BD7',
      yellow: '#E39F02',
      purple: '#AF298A',
      orange: '#E35B02',
      blue: '#2141C6',
      green: '#59B101',
      red: '#D70F37',
      garbage: '#686868',
    },
  }
settings?.openerPreviewColors && (hexes.preview = settings.openerPreviewColors)
class Piece {
  constructor(t, i, r, s = 0) {
    this.x = t
    this.y = i
    this.r = s
    this.color = r
  }
  show(t, i = 2) {
    'blue' == this.color
      ? (this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-1, -1, this.r)) * i * ratio,
          16 * (this.y + rY(-1, -1, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-1, 0, this.r)) * i * ratio,
          16 * (this.y + rY(-1, 0, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * this.x * i * ratio,
          16 * this.y * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(1, 0, this.r)) * i * ratio,
          16 * (this.y + rY(1, 0, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ))
      : 'orange' == this.color
      ? (this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(1, -1, this.r)) * i * ratio,
          16 * (this.y + rY(1, -1, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-1, 0, this.r)) * i * ratio,
          16 * (this.y + rY(-1, 0, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * this.x * i * ratio,
          16 * this.y * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(1, 0, this.r)) * i * ratio,
          16 * (this.y + rY(1, 0, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ))
      : 'green' == this.color
      ? (this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(1, -1, this.r)) * i * ratio,
          16 * (this.y + rY(1, -1, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-1, 0, this.r)) * i * ratio,
          16 * (this.y + rY(-1, 0, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * this.x * i * ratio,
          16 * this.y * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(0, -1, this.r)) * i * ratio,
          16 * (this.y + rY(0, -1, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ))
      : 'red' == this.color
      ? (this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-1, -1, this.r)) * i * ratio,
          16 * (this.y + rY(-1, -1, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(1, 0, this.r)) * i * ratio,
          16 * (this.y + rY(1, 0, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * this.x * i * ratio,
          16 * this.y * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(0, -1, this.r)) * i * ratio,
          16 * (this.y + rY(0, -1, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ))
      : 'yellow' == this.color
      ? (this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-0.5, -0.5, this.r) - 0.5) * i * ratio,
          16 * (this.y + rY(-0.5, -0.5, this.r) - 0.5) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-0.5, 0.5, this.r) - 0.5) * i * ratio,
          16 * (this.y + rY(-0.5, 0.5, this.r) - 0.5) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(0.5, 0.5, this.r) - 0.5) * i * ratio,
          16 * (this.y + rY(0.5, 0.5, this.r) - 0.5) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(0.5, -0.5, this.r) - 0.5) * i * ratio,
          16 * (this.y + rY(0.5, -0.5, this.r) - 0.5) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ))
      : 'teal' == this.color
      ? (this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-1.5, -0.5, this.r) - 0.5) * i * ratio,
          16 * (this.y + rY(-1.5, -0.5, this.r) - 0.5) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-0.5, -0.5, this.r) - 0.5) * i * ratio,
          16 * (this.y + rY(-0.5, -0.5, this.r) - 0.5) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(0.5, -0.5, this.r) - 0.5) * i * ratio,
          16 * (this.y + rY(0.5, -0.5, this.r) - 0.5) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(1.5, -0.5, this.r) - 0.5) * i * ratio,
          16 * (this.y + rY(1.5, -0.5, this.r) - 0.5) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ))
      : 'purple' == this.color &&
        (this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(-1, 0, this.r)) * i * ratio,
          16 * (this.y + rY(-1, 0, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(1, 0, this.r)) * i * ratio,
          16 * (this.y + rY(1, 0, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * this.x * i * ratio,
          16 * this.y * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ),
        this.sprite(
          t ? 'outline' : prefix,
          this.color,
          bx + 16 * (this.x + rX(0, -1, this.r)) * i * ratio,
          16 * (this.y + rY(0, -1, this.r)) * i * ratio,
          0,
          16 * i * ratio,
          16 * i * ratio
        ))
  }
  collide(t, i) {
    if ('blue' == this.color) {
      if (
        coll(this.x + rX(-1, -1, this.r) + t, this.y + rY(-1, -1, this.r) + i)
      ) {
        return true
      }
      if (
        coll(this.x + rX(-1, 0, this.r) + t, this.y + rY(-1, 0, this.r) + i)
      ) {
        return true
      }
      if (coll(this.x + rX(0, 0, this.r) + t, this.y + rY(0, 0, this.r) + i)) {
        return true
      }
      if (coll(this.x + rX(1, 0, this.r) + t, this.y + rY(1, 0, this.r) + i)) {
        return true
      }
    } else {
      if ('orange' == this.color) {
        if (
          coll(this.x + rX(1, -1, this.r) + t, this.y + rY(1, -1, this.r) + i)
        ) {
          return true
        }
        if (
          coll(this.x + rX(-1, 0, this.r) + t, this.y + rY(-1, 0, this.r) + i)
        ) {
          return true
        }
        if (
          coll(this.x + rX(0, 0, this.r) + t, this.y + rY(0, 0, this.r) + i)
        ) {
          return true
        }
        if (
          coll(this.x + rX(1, 0, this.r) + t, this.y + rY(1, 0, this.r) + i)
        ) {
          return true
        }
      } else {
        if ('green' == this.color) {
          if (
            coll(this.x + rX(1, -1, this.r) + t, this.y + rY(1, -1, this.r) + i)
          ) {
            return true
          }
          if (
            coll(this.x + rX(-1, 0, this.r) + t, this.y + rY(-1, 0, this.r) + i)
          ) {
            return true
          }
          if (
            coll(this.x + rX(0, 0, this.r) + t, this.y + rY(0, 0, this.r) + i)
          ) {
            return true
          }
          if (
            coll(this.x + rX(0, -1, this.r) + t, this.y + rY(0, -1, this.r) + i)
          ) {
            return true
          }
        } else {
          if ('red' == this.color) {
            if (
              coll(
                this.x + rX(-1, -1, this.r) + t,
                this.y + rY(-1, -1, this.r) + i
              )
            ) {
              return true
            }
            if (
              coll(this.x + rX(1, 0, this.r) + t, this.y + rY(1, 0, this.r) + i)
            ) {
              return true
            }
            if (
              coll(this.x + rX(0, 0, this.r) + t, this.y + rY(0, 0, this.r) + i)
            ) {
              return true
            }
            if (
              coll(
                this.x + rX(0, -1, this.r) + t,
                this.y + rY(0, -1, this.r) + i
              )
            ) {
              return true
            }
          } else {
            if ('yellow' == this.color) {
              if (
                coll(
                  this.x + rX(-0.5, -0.5, this.r) - 0.5 + t,
                  this.y + rY(-0.5, -0.5, this.r) - 0.5 + i
                )
              ) {
                return true
              }
              if (
                coll(
                  this.x + rX(-0.5, 0.5, this.r) - 0.5 + t,
                  this.y + rY(-0.5, 0.5, this.r) - 0.5 + i
                )
              ) {
                return true
              }
              if (
                coll(
                  this.x + rX(0.5, 0.5, this.r) - 0.5 + t,
                  this.y + rY(0.5, 0.5, this.r) - 0.5 + i
                )
              ) {
                return true
              }
              if (
                coll(
                  this.x + rX(0.5, -0.5, this.r) - 0.5 + t,
                  this.y + rY(0.5, -0.5, this.r) - 0.5 + i
                )
              ) {
                return true
              }
            } else {
              if ('teal' == this.color) {
                if (
                  coll(
                    this.x + rX(-1.5, -0.5, this.r) - 0.5 + t,
                    this.y + rY(-1.5, -0.5, this.r) - 0.5 + i
                  )
                ) {
                  return true
                }
                if (
                  coll(
                    this.x + rX(-0.5, -0.5, this.r) - 0.5 + t,
                    this.y + rY(-0.5, -0.5, this.r) - 0.5 + i
                  )
                ) {
                  return true
                }
                if (
                  coll(
                    this.x + rX(0.5, -0.5, this.r) - 0.5 + t,
                    this.y + rY(0.5, -0.5, this.r) - 0.5 + i
                  )
                ) {
                  return true
                }
                if (
                  coll(
                    this.x + rX(1.5, -0.5, this.r) - 0.5 + t,
                    this.y + rY(1.5, -0.5, this.r) - 0.5 + i
                  )
                ) {
                  return true
                }
              } else {
                if ('purple' == this.color) {
                  if (
                    coll(
                      this.x + rX(-1, 0, this.r) + t,
                      this.y + rY(-1, 0, this.r) + i
                    )
                  ) {
                    return true
                  }
                  if (
                    coll(
                      this.x + rX(1, 0, this.r) + t,
                      this.y + rY(1, 0, this.r) + i
                    )
                  ) {
                    return true
                  }
                  if (
                    coll(
                      this.x + rX(0, 0, this.r) + t,
                      this.y + rY(0, 0, this.r) + i
                    )
                  ) {
                    return true
                  }
                  if (
                    coll(
                      this.x + rX(0, -1, this.r) + t,
                      this.y + rY(0, -1, this.r) + i
                    )
                  ) {
                    return true
                  }
                }
              }
            }
          }
        }
      }
    }
    return false
  }
  setBlock() {
    'blue' == this.color
      ? ((board[this.x + rX(-1, -1, this.r)][this.y + rY(-1, -1, this.r)] =
          new Block(
            this.x + rX(-1, -1, this.r),
            this.y + rY(-1, -1, this.r),
            this.color
          )),
        (board[this.x + rX(-1, 0, this.r)][this.y + rY(-1, 0, this.r)] =
          new Block(
            this.x + rX(-1, 0, this.r),
            this.y + rY(-1, 0, this.r),
            this.color
          )),
        (board[this.x][this.y] = new Block(this.x, this.y, this.color)),
        (board[this.x + rX(1, 0, this.r)][this.y + rY(1, 0, this.r)] =
          new Block(
            this.x + rX(1, 0, this.r),
            this.y + rY(1, 0, this.r),
            this.color
          )))
      : 'orange' == this.color
      ? ((board[this.x + rX(1, -1, this.r)][this.y + rY(1, -1, this.r)] =
          new Block(
            this.x + rX(1, -1, this.r),
            this.y + rY(1, -1, this.r),
            this.color
          )),
        (board[this.x + rX(-1, 0, this.r)][this.y + rY(-1, 0, this.r)] =
          new Block(
            this.x + rX(-1, 0, this.r),
            this.y + rY(-1, 0, this.r),
            this.color
          )),
        (board[this.x][this.y] = new Block(this.x, this.y, this.color)),
        (board[this.x + rX(1, 0, this.r)][this.y + rY(1, 0, this.r)] =
          new Block(
            this.x + rX(1, 0, this.r),
            this.y + rY(1, 0, this.r),
            this.color
          )))
      : 'green' == this.color
      ? ((board[this.x + rX(1, -1, this.r)][this.y + rY(1, -1, this.r)] =
          new Block(
            this.x + rX(1, -1, this.r),
            this.y + rY(1, -1, this.r),
            this.color
          )),
        (board[this.x + rX(-1, 0, this.r)][this.y + rY(-1, 0, this.r)] =
          new Block(
            this.x + rX(-1, 0, this.r),
            this.y + rY(-1, 0, this.r),
            this.color
          )),
        (board[this.x][this.y] = new Block(this.x, this.y, this.color)),
        (board[this.x + rX(0, -1, this.r)][this.y + rY(0, -1, this.r)] =
          new Block(
            this.x + rX(0, -1, this.r),
            this.y + rY(0, -1, this.r),
            this.color
          )))
      : 'red' == this.color
      ? ((board[this.x + rX(-1, -1, this.r)][this.y + rY(-1, -1, this.r)] =
          new Block(
            this.x + rX(-1, -1, this.r),
            this.y + rY(-1, -1, this.r),
            this.color
          )),
        (board[this.x + rX(1, 0, this.r)][this.y + rY(1, 0, this.r)] =
          new Block(
            this.x + rX(1, 0, this.r),
            this.y + rY(1, 0, this.r),
            this.color
          )),
        (board[this.x][this.y] = new Block(this.x, this.y, this.color)),
        (board[this.x + rX(0, -1, this.r)][this.y + rY(0, -1, this.r)] =
          new Block(
            this.x + rX(0, -1, this.r),
            this.y + rY(0, -1, this.r),
            this.color
          )))
      : 'yellow' == this.color
      ? ((board[this.x + rX(-0.5, -0.5, this.r) - 0.5][
          this.y + rY(-0.5, -0.5, this.r) - 0.5
        ] = new Block(
          this.x + rX(-0.5, -0.5, this.r) - 0.5,
          this.y + rY(-0.5, -0.5, this.r) - 0.5,
          this.color
        )),
        (board[this.x + rX(-0.5, 0.5, this.r) - 0.5][
          this.y + rY(-0.5, 0.5, this.r) - 0.5
        ] = new Block(
          this.x + rX(-0.5, 0.5, this.r) - 0.5,
          this.y + rY(-0.5, 0.5, this.r) - 0.5,
          this.color
        )),
        (board[this.x + rX(0.5, 0.5, this.r) - 0.5][
          this.y + rY(0.5, 0.5, this.r) - 0.5
        ] = new Block(
          this.x + rX(0.5, 0.5, this.r) - 0.5,
          this.y + rY(0.5, 0.5, this.r) - 0.5,
          this.color
        )),
        (board[this.x + rX(0.5, -0.5, this.r) - 0.5][
          this.y + rY(0.5, -0.5, this.r) - 0.5
        ] = new Block(
          this.x + rX(0.5, -0.5, this.r) - 0.5,
          this.y + rY(0.5, -0.5, this.r) - 0.5,
          this.color
        )))
      : 'teal' == this.color
      ? ((board[this.x + rX(-1.5, -0.5, this.r) - 0.5][
          this.y + rY(-1.5, -0.5, this.r) - 0.5
        ] = new Block(
          this.x + rX(-1.5, -0.5, this.r) - 0.5,
          this.y + rY(-1.5, -0.5, this.r) - 0.5,
          this.color
        )),
        (board[this.x + rX(-0.5, -0.5, this.r) - 0.5][
          this.y + rY(-0.5, -0.5, this.r) - 0.5
        ] = new Block(
          this.x + rX(-0.5, -0.5, this.r) - 0.5,
          this.y + rY(-0.5, -0.5, this.r) - 0.5,
          this.color
        )),
        (board[this.x + rX(0.5, -0.5, this.r) - 0.5][
          this.y + rY(0.5, -0.5, this.r) - 0.5
        ] = new Block(
          this.x + rX(0.5, -0.5, this.r) - 0.5,
          this.y + rY(0.5, -0.5, this.r) - 0.5,
          this.color
        )),
        (board[this.x + rX(1.5, -0.5, this.r) - 0.5][
          this.y + rY(1.5, -0.5, this.r) - 0.5
        ] = new Block(
          this.x + rX(1.5, -0.5, this.r) - 0.5,
          this.y + rY(1.5, -0.5, this.r) - 0.5,
          this.color
        )))
      : 'purple' == this.color &&
        ((board[this.x + rX(-1, 0, this.r)][this.y + rY(-1, 0, this.r)] =
          new Block(
            this.x + rX(-1, 0, this.r),
            this.y + rY(-1, 0, this.r),
            this.color
          )),
        (board[this.x + rX(1, 0, this.r)][this.y + rY(1, 0, this.r)] =
          new Block(
            this.x + rX(1, 0, this.r),
            this.y + rY(1, 0, this.r),
            this.color
          )),
        (board[this.x][this.y] = new Block(this.x, this.y, this.color)),
        (board[this.x + rX(0, -1, this.r)][this.y + rY(0, -1, this.r)] =
          new Block(
            this.x + rX(0, -1, this.r),
            this.y + rY(0, -1, this.r),
            this.color
          )))
    blockId++
  }
  sprite(i, r, s = 0, h = 0, t, o = 16, e = 16) {
    if (presetSkins.includes(i)) {
      settings.webgl
        ? ctx != hCtx &&
          ctx != qCtx &&
          ctx != bCtx &&
          ctx != rCtx &&
          drawImage(images[i + cols[r]], s, h, o, e)
        : ((ctx.fillStyle = hexes[i][r]), ctx.fillRect(s, h, o, e))
    } else {
      if (
        'undefined' != typeof settings &&
        settings &&
        settings.skinConnected &&
        'custom' == i
      ) {
        var l = sprites[i],
          a = (l || console.log(i), board),
          x =
            (((board = createBoard())[-1] = []),
            (board[-2] = []),
            (board[-3] = []),
            (board[bW] = []),
            this.setBlock(),
            22)
        let t = 4 * (cols[r] - 2)
        3 < cols[r] - 2 && (t += 116)
        var c = s / (32 * ratio),
          p = h / (32 * ratio)
        this.pAt(c - 1, p) ||
        this.pAt(1 + c, p) ||
        this.pAt(c, p - 1) ||
        !this.pAt(c, 1 + p)
          ? !this.pAt(c - 1, p) &&
            !this.pAt(1 + c, p) &&
            this.pAt(c, p - 1) &&
            this.pAt(c, 1 + p)
            ? (t += x)
            : this.pAt(c - 1, p) ||
              this.pAt(1 + c, p) ||
              !this.pAt(c, p - 1) ||
              this.pAt(c, 1 + p)
            ? this.pAt(c - 1, p) ||
              !this.pAt(1 + c, p) ||
              this.pAt(c, p - 1) ||
              this.pAt(c, 1 + p)
              ? this.pAt(c - 1, p) &&
                this.pAt(1 + c, p) &&
                !this.pAt(c, p - 1) &&
                !this.pAt(c, 1 + p)
                ? (t += 68)
                : !this.pAt(c - 1, p) ||
                  this.pAt(1 + c, p) ||
                  this.pAt(c, p - 1) ||
                  this.pAt(c, 1 + p)
                ? this.pAt(c - 1, p) ||
                  this.pAt(1 + c, p) ||
                  this.pAt(c, p - 1) ||
                  this.pAt(c, 1 + p)
                  ? !this.pAt(c - 1, p) &&
                    this.pAt(1 + c, p) &&
                    !this.pAt(c, p - 1) &&
                    this.pAt(c, 1 + p)
                    ? (t += 88)
                    : this.pAt(c - 1, p) &&
                      !this.pAt(1 + c, p) &&
                      !this.pAt(c, p - 1) &&
                      this.pAt(c, 1 + p)
                    ? (t += 89)
                    : !this.pAt(c - 1, p) &&
                      this.pAt(1 + c, p) &&
                      this.pAt(c, p - 1) &&
                      !this.pAt(c, 1 + p)
                    ? (t += 110)
                    : this.pAt(c - 1, p) &&
                      !this.pAt(1 + c, p) &&
                      this.pAt(c, p - 1) &&
                      !this.pAt(c, 1 + p)
                    ? (t += 111)
                    : this.pAt(c - 1, p) &&
                      this.pAt(1 + c, p) &&
                      this.pAt(c, p - 1) &&
                      !this.pAt(c, 1 + p)
                    ? (t += 90)
                    : !this.pAt(c - 1, p) &&
                      this.pAt(1 + c, p) &&
                      this.pAt(c, p - 1) &&
                      this.pAt(c, 1 + p)
                    ? (t += 91)
                    : this.pAt(c - 1, p) &&
                      this.pAt(1 + c, p) &&
                      !this.pAt(c, p - 1) &&
                      this.pAt(c, 1 + p)
                    ? (t += 112)
                    : this.pAt(c - 1, p) &&
                      !this.pAt(1 + c, p) &&
                      this.pAt(c, p - 1) &&
                      this.pAt(c, 1 + p) &&
                      (t += 113)
                  : (t += 66)
                : (t += 69)
              : (t += 67)
            : (t += 44)
          : (t += 0)
        x = l.sprites[t % l.sprites.length]
        try {
          ctx.drawImage(x, 0, 0, l.w, l.h, s, h, o, e)
        } catch (t) {}
        board = a
      } else {
        c = sprites[i]
        c || console.log(i)
        let t
        t =
          'outline' != i
            ? c.sprites[cols[r] % c.sprites.length]
            : c.sprites[Math.max(cols[r] - 2, 0) % c.sprites.length]
        ctx.drawImage(t, 0, 0, c.w, c.h, s, h, o, e)
      }
    }
  }
  pAt(t, i) {
    return board[t][i]
  }
}
