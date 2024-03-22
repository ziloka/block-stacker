class Computer {
    constructor(i, t, s, e = 'Computer') {
      this.type = i
      this.canvas = t
      this.board = new Board(t)
      ;(this.board.player = this).board.target = s
      this.worker = new Worker('/js/computerWorker.js')
      let h = this
      this.worker.onmessage = function (i) {
        h.searching = false
        null != i.data
          ? (h.moves = i.data)
          : (h.board.endGame(), this.terminate())
      }
      this.delay =
        {
          'extremely-easy': 800,
          'very-easy': 400,
          easy: 200,
          medium: 100,
          hard: 50,
          extreme: 25,
          impossible: 1,
          match: 1000000,
        }[i] || 200
      this.nextTime = performance.now() + 1000
      this.moves = []
      this.depth = 4
      this.combinations = []
      this.maxComb = 20000
      this.shitLimit = 10
      this.goodLimit = 5
      this.searching = false
      this.board.name = e
    }
    update() {
      if (this.board.alive) {
        if (
          this.nextTime < performance.now() ||
          (this.board.pieces <= pieces && 'match' == this.type)
        ) {
          if (
            (0 == this.moves.length &&
              0 == this.searching &&
              (this.worker.postMessage({
                type: this.type,
                depth: this.depth,
                board: {
                  b: this.board.b,
                  holdBlock: this.board.holdBlock,
                  piece: this.board.piece,
                  queue: this.board.queue,
                },
              }),
              (this.searching = true)),
            'impossible' == this.type || 'match' == this.type)
          ) {
            for (; 0 < this.moves.length; ) {
              var i = this.moves.shift()
              this.board.action(i)
            }
          } else {
            var t = this.moves.shift()
            this.board.action(t)
          }
          this.nextTime = performance.now() + this.delay
        }
        this.board.updateBoard()
      }
    }
    analyze() {
      this.shitLimit = -20
      let i = [this.board.piece].concat(this.board.queue.slice())
      var t = this.board.holdBlock,
        s = JSON.parse(JSON.stringify(this.board.b))
      let e = this.calcBoard(s, 0)
      'number' != typeof e && (e = 0)
      this.combinations = []
      for (let i = 0; i < this.depth + 1; i++) {
        this.combinations.push(0)
      }
      var h = this.processMoves(this.depth, s, i, t, e)
      let r
      var o = i[0]
      this.combinations = []
      for (let i = 0; i < this.depth + 1; i++) {
        this.combinations.push(0)
      }
      let l = i.slice()
      ;(r =
        (t ? (l[0] = t) : l.shift(),
        this.processMoves(this.depth, s, l, o, e))) &&
      h &&
      r.score < h.score
        ? (this.moves = [10].concat(r.moves))
        : h
        ? (this.moves = h.moves)
        : this.board.endGame()
    }
    processMoves(h, o, i, r, t) {
      if (!(this.combinations[h] < this.maxComb)) {
        return null
      }
      this.combinations[h]++
      let l = i.slice(),
        c = [],
        a = []
      var n = l[0]
      if ((l.splice(0, 1), !n)) {
        return null
      }
      var s = 'teal' == n.color ? 11 : 9
      for (
        let i = 0;
        i < 4 && (h == this.depth || this.combinations[h - 1] != this.maxComb);
        i++
      ) {
        for (
          let r = 0;
          r < s && (h == this.depth || this.combinations[h - 1] != this.maxComb);
          r++
        ) {
          let t = new PieceB(n.x, n.y, n.color),
            s = [],
            e = (0 != (t.r = i) && s.push([0, 4, 6, 5][i]), [])
          for (let i = 0; i < o.length; i++) {
            e[i] = o[i].slice()
          }
          let h = false
          for (let i = 0; i < Math.ceil(r / 2); i++) {
            if (t.collide(r % 2 == 0 ? -1 : 1, 0, e)) {
              h = true
              break
            }
            s.push(r % 2 == 0 ? 0 : 1)
            t.x += r % 2 == 0 ? -1 : 1
          }
          if (!h) {
            for (; t.y < 20 && !t.collide(0, 1, e); ) {
              t.y++
            }
            t.setBlock(e)
            s.push(7)
            var d = this.calcBoard(e, t.y)
            a.push({
              score: d,
              moves: s,
              brd: e,
            })
          }
        }
      }
      if (0 != c.length || this.combinations[h - 1] == this.maxComb) {
        return c[bestBoard] || null
      }
      {
        c = []
        a.sort((i, t) => i.score - t.score)
        for (let i = 0; i < this.goodLimit; i++) {
          c.push(a[i])
        }
        let i = -1,
          e = 1e400
        for (let s = 0; s < c.length; s++) {
          if (c[s]) {
            let t = c[s].score
            if (1 < h && this.combinations[h - 1] < this.maxComb) {
              var p = this.processMoves(h - 1, c[s].brd, l, r, c[s].score)
              let i = l.slice()
              r ? (i[0] = r) : i.shift()
              var b = this.processMoves(h - 1, c[s].brd, i, l[0], c[s].score)
              b && (!p || b.score < p.score)
                ? (t = (t + b.score) / 2)
                : p && (t = (t + p.score) / 2)
            }
            t < e && ((i = s), (e = t))
          }
        }
        return c[i] || null
      }
    }
    calcBoard(h, i) {
      let e = 0
      for (let s = i - 2; s < i + 2; s++) {
        let t = 0
        for (let i = 0; i < 10; i++) {
          void 0 !== h[i][s] && null !== h[i][s] && t++
        }
        if (10 == t) {
          e++
          for (let t = 0; t < 10; t++) {
            for (let i = s; 20 <= i; i--) {
              ;-20 < i ? (h[t][i] = h[t][i - 1]) : delete h[t][i]
            }
          }
        }
      }
      let r = 0,
        o = 0
      let l = 0,
        c = [],
        a = 0
      for (let e = 0; e < 10; e++) {
        let t = false,
          s = false
        for (let i = 0; i < 20; i++) {
          void 0 !== h[e][i] && null !== h[e][i]
            ? ((t = t || true),
              s || ((s = true), 20 - i > a && (a = 20 - i), c.push(20 - i)),
              r++,
              (o += 20 - i))
            : t && l++
        }
        s || c.push(0)
      }
      let t = 0,
        s = 0
      c[-1] = 20
      c[10] = 20
      for (let i = 0; i < 10; i++) {
        0 < i && 2 <= c[i] - c[i - 1] && (s += c[i] - c[i - 1] - 1)
        i < 9 && 2 <= c[i] - c[i + 1] && (s += c[i] - c[i + 1] - 1)
        0 < i && 2 <= c[i - 1] - c[i] && (t += c[i - 1] - c[i] - 1)
        i < 9 && 2 <= c[i + 1] - c[i] && (t += c[i + 1] - c[i] - 1)
      }
      4 == e && (e = -20)
      e *= 10
      ;(8 < a || 0 < l) && 0 < e && (e *= -1)
      let n = 0
      if (
        (n =
          8 < a || 0 < l
            ? (o / r) * 2 + o / 20 + 16 * l + 3 * s + t + 0
            : (o / r) * 0.5 + 10 * l + +t + e)
      ) {
        return n
      }
    }
  }
  function Board(t) {
    this.player = null
    this.b = JSON.parse(JSON.stringify(createBoard()))
    this.seed = this.gameStart = this.curTime = c.seed
    this.m = mode
    this.settings = {
      startSpeed: 1,
      speedIncrease: 1,
      speedLimit: 0,
      rotationSystem: 'SRS',
      mode: mode,
      attackTable: [0, 1, 2, 4, 2, 4, 6, 10],
      comboTable: [0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 4, 5],
      infiniteHold: false,
    }
    1 == this.m && (this.settings.mode = 2)
    3 == this.m && (this.settings.mode = 3)
    4 == this.m && (this.settings.mode = 4)
    8 == this.m && (this.settings.mode = 1)
    this.alive = true
    this.won = false
    this.updating = false
    this.prevPiece = null
    this.pieces = 0
    this.switched = false
    this.holdBlock = null
    this.lines = 0
    this.tempHeight = 4
    this.prevPoints = this.points = 0
    this.combo = this.pcs = 0
    this.actions = []
    this.attack = 0
    this.garbageStore = []
    this.garbageAmount = 0
    this.garbageHeight = 0
    this.lastSender = ''
    this.glines = 0
    this.prevGarb = 10
    this.max_lines = 10
    this.gRNG = null
    this.canvas = t
    this.start = function () {
      this.gameStart = new Date().getTime()
      this.bag = new Bag(alea(this.seed), 7, 1)
      this.gRNG = alea(this.seed)
      this.queue = []
      for (let i = 0; i < 6; i++) {
        this.addBlock()
      }
      var i
      5 <= colors.indexOf(this.queue[0].color) &&
        2 == this.settings.mode &&
        ((i = this.queue[0]),
        colors.indexOf(this.queue[1].color) < 5
          ? ((this.queue[0] = this.queue[1]), (this.queue[1] = i))
          : ((this.queue[0] = this.queue[2]), (this.queue[2] = i)))
      this.piece = this.queue.splice(0, 1)[0]
      2 == this.settings.mode && (this.max_lines = 40)
      3 == this.settings.mode &&
        ((this.glines = 9), (this.max_lines = 10), (this.prevGarb = 10))
      this.alive = false
      this.updating = false
      this.alive = true
      this.updateBoard()
    }
    this.action = function (e, h) {
      if (this.alive && 0 <= e && e <= 22) {
        this.curTime = new Date().getTime()
        let i = {
            t: this.curTime - this.gameStart,
            a: Math.min(e, 15),
          },
          t =
            (h && (i.d = h), 15 < e && (i.aux = e % 16), this.actions.push(i), 0),
          s = false
        switch (e) {
          case 0:
            this.piece.collide(-1, 0, this.b) || this.piece.x--
            break
          case 1:
            this.piece.collide(1, 0, this.b) || this.piece.x++
            break
          case 2:
            for (; !this.piece.collide(-1, 0, this.b); ) {
              this.piece.x--
            }
            break
          case 3:
            for (; !this.piece.collide(1, 0, this.b); ) {
              this.piece.x++
            }
            break
          case 4:
            ;(t = this.piece.r),
              (this.piece.r = (this.piece.r + 1) % 4),
              (s = this.collisionRoutine(
                this.piece,
                t,
                this.settings.rotationSystem,
                this.b
              )) && (this.piece.r = (this.piece.r - 1 + 4) % 4)
            break
          case 5:
            ;(t = this.piece.r),
              (this.piece.r = (this.piece.r - 1 + 4) % 4),
              (s = this.collisionRoutine(
                this.piece,
                t,
                this.settings.rotationSystem,
                this.b
              )) && (this.piece.r = (this.piece.r + 1) % 4)
            break
          case 6:
            ;(t = this.piece.r),
              (this.piece.r = (this.piece.r + 2) % 4),
              (s = this.collisionRoutine(
                this.piece,
                t,
                this.settings.rotationSystem,
                this.b
              )) && (this.piece.r = (this.piece.r + 2) % 4)
            break
          case 7:
            for (; this.piece.y < 20 && !this.piece.collide(0, 1, this.b); ) {
              this.piece.y++
            }
            ;(this.prevPiece = this.piece),
              this.piece.setBlock(this.b),
              (this.piece = this.queue.splice(0, 1)[0]),
              this.addBlock(),
              (this.switched = false),
              this.piece.collide(0, 0, this.b) && this.endGame()
            break
          case 8:
            break
          case 9:
            this.piece.collide(0, 1, this.b) || this.piece.y++
            break
          case 10:
            var r
            0 == this.switched &&
              ((this.piece.x =
                'teal' == this.piece.color || 'yellow' == this.piece.color
                  ? 5
                  : 4),
              (this.piece.y = this.piece.r = 0),
              null == this.holdBlock
                ? ((this.holdBlock = this.piece),
                  (this.piece = this.queue.splice(0, 1)[0]),
                  this.addBlock())
                : ((r = this.holdBlock),
                  (this.holdBlock = this.piece),
                  (this.piece = r)),
              this.settings.infiniteHold
                ? (this.switched = false)
                : (this.switched = true))
            break
          case 11:
            this.garbage(h[0], h[1]),
              this.piece.collide(0, 0, this.b) && this.endGame()
            break
          case 21:
            this.garbage(h[0], h[1], h[2])
            break
          case 22:
            this.setGarbage(h[0], h[1], h[2])
        }
      }
    }
    this.updateBoard = function () {
      this.updating = true
      let t = 0,
        h =
          (this.prevPiece &&
            'purple' == this.prevPiece.color &&
            (3 != this.prevPiece.r &&
              (0 < this.prevPiece.x
                ? this.b[this.prevPiece.x - 1][this.prevPiece.y - 1] && t++
                : this.prevPiece.x - 1 < 0 && t++),
            1 != this.prevPiece.r &&
              (this.prevPiece.x < 9
                ? this.b[this.prevPiece.x + 1][this.prevPiece.y - 1] && t++
                : 9 < this.prevPiece.x + 1 && t++),
            0 < this.prevPiece.x
              ? this.b[this.prevPiece.x - 1][this.prevPiece.y + 1] && t++
              : this.prevPiece.x - 1 < 0 && t++,
            this.prevPiece.x < 9
              ? this.b[this.prevPiece.x + 1][this.prevPiece.y + 1] && t++
              : 9 < this.prevPiece.x + 1 && t++),
          0)
      for (let e = -20; e < 20; e++) {
        let t = 0,
          s = false
        for (i = 0; i < 10 && this.b[i][e]; i++) {
          t++
          'garbage' == this.b[i][e].color && (s = true)
        }
        if (10 == t) {
          for (let i = 0; i < 10; i++) {
            delete this.b[i][e]
          }
          h++
          s && this.garbageHeight--
          for (let t = e; -20 <= t; t--) {
            for (i = 0; i < 10; i++) {
              ;-20 < t
                ? (this.b[i][t - 1] && this.b[i][t - 1].y++,
                  (this.b[i][t] = this.b[i][t - 1]))
                : delete this.b[i][t]
            }
          }
        }
      }
      if (
        ((this.prevPoints = this.points),
        3 <= t
          ? (1 == h && (this.points += this.settings.attackTable[4]),
            2 == h && (this.points += this.settings.attackTable[5]),
            3 == h && (this.points += this.settings.attackTable[6]))
          : (1 == h && (this.points += this.settings.attackTable[0]),
            2 == h && (this.points += this.settings.attackTable[1]),
            3 == h && (this.points += this.settings.attackTable[2]),
            4 == h && (this.points += this.settings.attackTable[3])),
        0 < h)
      ) {
        this.lines += h
        this.tempHeight -= h
        let s = true
        for (let t = 0; t < 10; t++) {
          for (let i = 0; i < 20; i++) {
            this.b[t][i] && (s = false)
          }
        }
        if (
          (s &&
            ((this.points += this.settings.attackTable[7]),
            1 == this.settings.mode && ((this.tempHeight = 4), this.pcs++)),
          this.combo < 12
            ? (this.points += this.settings.comboTable[this.combo])
            : (this.points += this.settings.comboTable[11]),
          this.combo++,
          6 == this.settings.mode)
        ) {
          for (let t = 1; t < 1 + h; t++) {
            for (let i = 0; i < 6; i++) {
              2 < i
                ? (this.b[i + 4][t] = new Block(i, t, 'garbage'))
                : (this.b[i][t] = new Block(i, t, 'garbage'))
            }
          }
          this.action(22, [h, 3, 4, 0])
        }
      } else {
        null != this.prevPiece &&
          (6 == this.settings.mode ? this.endGame() : (this.combo = 0))
      }
      if (null != this.prevPiece && 1 == this.settings.mode) {
        let t = true
        for (let i = 0; i < 10; i++) {
          this.b[i][19 - this.tempHeight] && (t = false)
        }
        t || this.endGame()
      }
      let s = this.points - this.prevPoints
      if (null != this.prevPiece && this.alive) {
        if (0 < h) {
          for (let i = 0; i < this.garbageStore.length; i++) {
            if (!(s >= this.garbageStore[i].num)) {
              this.garbageStore[i].num -= s
              this.garbageAmount -= s
              break
            }
            if (
              ((s -= this.garbageStore[i].num),
              (this.garbageAmount -= this.garbageStore[i].num),
              this.garbageStore.splice(i, 1),
              0 == s)
            ) {
              break
            }
          }
        } else {
          for (let i = 0; i < this.garbageStore.length; i++) {
            this.action(11, [this.garbageStore[i].num, this.garbageStore[i].rand])
            this.lastSender = this.garbageStore[i].send
            this.garbageAmount -= this.garbageStore[i].num
            this.garbageStore.splice(i, 1)
            i--
          }
        }
      }
      if (
        (null != this.prevPiece && this.pieces++,
        (this.attack += this.points - this.prevPoints),
        this.alive &&
          this.piece.collide(0, 0, this.b) &&
          null != this.prevPiece &&
          this.endGame(),
        this.alive && 0 == this.settings.mode && 0 < s && this.send(s),
        2 == this.settings.mode && 40 <= this.lines && this.endGame(true),
        3 == this.settings.mode)
      ) {
        let s = 0,
          e = 0,
          i = 0
        for (let t = 0; t < 9; t++) {
          s = 0
          for (let i = (e = 0); i < 10; i++) {
            this.b[i][11 + t] ? 'garbage' == this.b[i][11 + t].color && e++ : s++
          }
          if (1 == s && 9 == e) {
            i = 9 - t
            break
          }
        }
        0 == i && this.endGame(true)
        s = 0
        for (let i = (e = 0); i < 10; i++) {
          this.b[i][10] ? 'garbage' == this.b[i][10].color && e++ : s++
        }
        let h = 1 == s && 9 == e
        for (; this.glines < this.max_lines && i < 9 && !h; ) {
          this.glines++
          i++
          let t = []
          for (let i = 0; i < 10; i++) {
            t[i] = i
          }
          var r = t.slice(0, this.prevGarb).concat(t.slice(this.prevGarb + 1, 10))
          this.prevGarb = r[Math.floor(this.gRNG() * r.length)]
          this.action(11, [1, this.prevGarb])
          s = 0
          for (let i = (e = 0); i < 10; i++) {
            this.b[i][10] ? 'garbage' == this.b[i][10].color && e++ : s++
          }
          h = 1 == s && 9 == e
        }
      }
      this.prevPiece = null
      this.updating = false
      let e = []
      for (i = 0; i < 10; i++) {
        e[i] = []
        for (let t = 0; t < 20; t++) {
          this.b[i][t]
            ? settings.skinConnected
              ? (e[i][t] = this.b[i][t])
              : (e[i][t] = this.b[i][t].color)
            : (e[i][t] = null)
        }
      }
      this.canvas.board = e
      this.canvas.ghost = this.piece
      drawBoard(this.canvas)
    }
    this.endGame = function (i) {
      if (this.alive) {
        this.alive = false
        this.won = !!i
        let t = 0,
          s = -1
        for (let i = 0; i < computers.length; i++) {
          computers[i].board.alive && (t++, (s = i))
        }
        if (-1 != s) {
          let i = '' + (t + 1 + (aiBattle ? 0 : 1))
          i.endsWith(1)
            ? (i += 'st')
            : i.endsWith(2)
            ? (i += 'nd')
            : i.endsWith(3)
            ? (i += 'rd')
            : (i += 'th')
          this.canvas.position = i
          1 == t &&
            computers[s] &&
            aiBattle &&
            ((computers[s].canvas.position = '1st'),
            (computers[s].winner = true),
            (computers[s].alive = false),
            drawBoard(computers[s].canvas))
        } else {
          this.canvas.position = '2nd'
          for (let i = 0; i < localBoards.length; i++) {
            localBoards[i].alive && (localBoards[i].winner = true)
          }
        }
        drawBoard(this.canvas)
        this.player.worker.terminate()
      }
    }
    this.update = function () {
      this.alive
    }
    this.getRandomizerBlock = function () {
      var i = this.bag.getBlock()
      return 'teal' == colors[i] || 'yellow' == colors[i]
        ? new PieceB(5, 0, colors[i])
        : new PieceB(4, 0, colors[i])
    }
    this.addBlock = function () {
      this.queue.push(this.getRandomizerBlock())
    }
    this.send = function (t) {
      if (this.alive) {
        let i = localBoards.filter((i) => i != this)[
          Math.floor(Math.random() * (localBoards.length - 1))
        ]
        i.garbageStore.push({
          num: t,
          rand: Math.floor(10 * Math.random()),
          send: this.name,
          time: performance.now(),
        })
        i.garbageAmount += t
      }
    }
    this.garbage = function (s, e, h = 1) {
      if (this.alive) {
        for (let t = 0; t < 20; t++) {
          for (let i = 0; i < 10; i++) {
            t < 20 - s
              ? (this.b[i][t + s] && (this.b[i][t + s].y += s),
                (this.b[i][t] = this.b[i][t + s]))
              : delete this.b[i][t]
          }
        }
        for (let t = 0; t < s; t++) {
          for (let i = 0; i < 10; i++) {
            i < e || i - e >= h
              ? (this.b[i][19 - t] = new Block(i, 19 - t, 'garbage'))
              : delete this.b[i][19 - t]
          }
        }
        this.garbageHeight += s
        this.piece.y -= s
        blockId++
      }
    }
    this.setGarbage = function (i, s, e = 1) {
      if (this.alive) {
        for (let t = 0; t < i; t++) {
          for (let i = 0; i < 10; i++) {
            i < s || i - s >= e
              ? (this.b[i][1 + t] = new Block(i, 1 + t, 'garbage'))
              : delete this.b[i][1 + t]
          }
        }
      }
    }
    this.encodeReplay = function (i) {
      let t = {
          v: 3.3,
          softDropId: 2,
          gameStart: this.gameStart,
          gameEnd: this.gameStart + i,
          seed: this.seed,
          m: 1,
          bs: 0,
          se: 2,
          das: 133,
          r: 0,
        },
        s =
          (1 == this.settings.mode
            ? (t.m = 8 << 16)
            : 2 == this.settings.mode
            ? (t.m = 65537)
            : 3 == this.settings.mode
            ? (t.m = 196609)
            : 4 == this.settings.mode
            ? (t.m = 4 << 16)
            : (t.m = 2 << 16),
          new Replayer())
      s.actions = this.actions
      i = s.saveBinaryActionsV3()
      i = {
        c: t,
        d: i,
      }
      return LZString.compressToEncodedURIComponent(JSON.stringify(i))
    }
    this.collisionRoutine = function (t, i, s, e) {
      let h = true
      if ('BRS' == s) {
        h =
          (h =
            (h =
              (h =
                (h =
                  (h =
                    (h = h && this.collide(t, 0, 0, e)) &&
                    this.collide(t, 0, 1, e)) && this.collide(t, 1, 1, e)) &&
                this.collide(t, -1, 1, e)) && this.collide(t, 0, 2, e)) &&
            this.collide(t, 1, 2, e)) && this.collide(t, -1, 2, e)
        h =
          (h =
            (h =
              (h =
                (h =
                  (h =
                    (h =
                      (h =
                        (h =
                          (h =
                            (h =
                              (h =
                                'teal' == t.color
                                  ? (h =
                                      (h =
                                        (h = h && this.collide(t, 2, 1, e)) &&
                                        this.collide(t, -2, 1, e)) &&
                                      this.collide(t, 2, 2, e)) &&
                                    this.collide(t, -2, 2, e)
                                  : h) && this.collide(t, 1, 0, e)) &&
                            this.collide(t, -1, 0, e)) &&
                          this.collide(t, 0, -1, e)) &&
                        this.collide(t, 1, -1, e)) &&
                      this.collide(t, -1, -1, e)) && this.collide(t, 2, 0, e)) &&
                  this.collide(t, -2, 0, e)) && this.collide(t, 2, 1, e)) &&
              this.collide(t, -2, 1, e)) && this.collide(t, 2, -1, e)) &&
          this.collide(t, -2, -1, e)
        'teal' == t.color &&
          (h =
            (h =
              (h = h && this.collide(t, 0, -2, e)) &&
              this.collide(t, 1, -2, e)) && this.collide(t, -1, -2, e))
      } else {
        if ('SRS' == s && (h = h && this.collide(t, 0, 0, e))) {
          var r = i,
            o = t.r
          if (2 == Math.abs(o - r)) {
            for (
              let i = 0;
              i < 2 && (h = this.collideSRS(t, spins180[r][i], e));
              i++
            ) {}
          } else {
            for (
              let i = 0;
              i < 5 &&
              ('teal' == t.color
                ? (0 == r && 3 == o && (h = this.collideSRS(t, iSpins[0][i], e)),
                  3 == r && 0 == o && (h = this.collideSRS(t, iSpins[1][i], e)),
                  3 == r && 2 == o && (h = this.collideSRS(t, iSpins[2][i], e)),
                  2 == r && 3 == o && (h = this.collideSRS(t, iSpins[3][i], e)),
                  2 == r && 1 == o && (h = this.collideSRS(t, iSpins[4][i], e)),
                  1 == r && 2 == o && (h = this.collideSRS(t, iSpins[5][i], e)),
                  1 == r && 0 == o && (h = this.collideSRS(t, iSpins[6][i], e)),
                  0 == r && 1 == o && (h = this.collideSRS(t, iSpins[7][i], e)))
                : (0 == r &&
                    3 == o &&
                    (h = this.collideSRS(t, otherSpins[0][i], e)),
                  3 == r &&
                    0 == o &&
                    (h = this.collideSRS(t, otherSpins[1][i], e)),
                  3 == r &&
                    2 == o &&
                    (h = this.collideSRS(t, otherSpins[2][i], e)),
                  2 == r &&
                    3 == o &&
                    (h = this.collideSRS(t, otherSpins[3][i], e)),
                  2 == r &&
                    1 == o &&
                    (h = this.collideSRS(t, otherSpins[4][i], e)),
                  1 == r &&
                    2 == o &&
                    (h = this.collideSRS(t, otherSpins[5][i], e)),
                  1 == r &&
                    0 == o &&
                    (h = this.collideSRS(t, otherSpins[6][i], e)),
                  0 == r &&
                    1 == o &&
                    (h = this.collideSRS(t, otherSpins[7][i], e))),
              h);
              i++
            ) {}
          }
        }
      }
      return h
    }
    this.collide = function (i, t, s, e) {
      return !!i.collide(t, s, e) || ((i.x += t), (i.y += s), false)
    }
    this.collideSRS = function (i, t, s) {
      return (
        !!i.collide(t[0], -1 * t[1], s) ||
        ((i.x += t[0]), (i.y += -1 * t[1]), false)
      )
    }
    this.start()
  }
  class PieceB {
    constructor(i, t, s) {
      this.x = i
      this.y = t
      this.r = 0
      this.color = s
    }
    update() {}
    collide(i, t, s) {
      if ('blue' == this.color) {
        if (
          this.coll(
            this.x + rX(-1, -1, this.r) + i,
            this.y + rY(-1, -1, this.r) + t,
            s
          )
        ) {
          return true
        }
        if (
          this.coll(
            this.x + rX(-1, 0, this.r) + i,
            this.y + rY(-1, 0, this.r) + t,
            s
          )
        ) {
          return true
        }
        if (
          this.coll(
            this.x + rX(0, 0, this.r) + i,
            this.y + rY(0, 0, this.r) + t,
            s
          )
        ) {
          return true
        }
        if (
          this.coll(
            this.x + rX(1, 0, this.r) + i,
            this.y + rY(1, 0, this.r) + t,
            s
          )
        ) {
          return true
        }
      } else {
        if ('orange' == this.color) {
          if (
            this.coll(
              this.x + rX(1, -1, this.r) + i,
              this.y + rY(1, -1, this.r) + t,
              s
            )
          ) {
            return true
          }
          if (
            this.coll(
              this.x + rX(-1, 0, this.r) + i,
              this.y + rY(-1, 0, this.r) + t,
              s
            )
          ) {
            return true
          }
          if (
            this.coll(
              this.x + rX(0, 0, this.r) + i,
              this.y + rY(0, 0, this.r) + t,
              s
            )
          ) {
            return true
          }
          if (
            this.coll(
              this.x + rX(1, 0, this.r) + i,
              this.y + rY(1, 0, this.r) + t,
              s
            )
          ) {
            return true
          }
        } else {
          if ('green' == this.color) {
            if (
              this.coll(
                this.x + rX(1, -1, this.r) + i,
                this.y + rY(1, -1, this.r) + t,
                s
              )
            ) {
              return true
            }
            if (
              this.coll(
                this.x + rX(-1, 0, this.r) + i,
                this.y + rY(-1, 0, this.r) + t,
                s
              )
            ) {
              return true
            }
            if (
              this.coll(
                this.x + rX(0, 0, this.r) + i,
                this.y + rY(0, 0, this.r) + t,
                s
              )
            ) {
              return true
            }
            if (
              this.coll(
                this.x + rX(0, -1, this.r) + i,
                this.y + rY(0, -1, this.r) + t,
                s
              )
            ) {
              return true
            }
          } else {
            if ('red' == this.color) {
              if (
                this.coll(
                  this.x + rX(-1, -1, this.r) + i,
                  this.y + rY(-1, -1, this.r) + t,
                  s
                )
              ) {
                return true
              }
              if (
                this.coll(
                  this.x + rX(1, 0, this.r) + i,
                  this.y + rY(1, 0, this.r) + t,
                  s
                )
              ) {
                return true
              }
              if (
                this.coll(
                  this.x + rX(0, 0, this.r) + i,
                  this.y + rY(0, 0, this.r) + t,
                  s
                )
              ) {
                return true
              }
              if (
                this.coll(
                  this.x + rX(0, -1, this.r) + i,
                  this.y + rY(0, -1, this.r) + t,
                  s
                )
              ) {
                return true
              }
            } else {
              if ('yellow' == this.color) {
                if (
                  this.coll(
                    this.x + rX(-0.5, -0.5, this.r) - 0.5 + i,
                    this.y + rY(-0.5, -0.5, this.r) - 0.5 + t,
                    s
                  )
                ) {
                  return true
                }
                if (
                  this.coll(
                    this.x + rX(-0.5, 0.5, this.r) - 0.5 + i,
                    this.y + rY(-0.5, 0.5, this.r) - 0.5 + t,
                    s
                  )
                ) {
                  return true
                }
                if (
                  this.coll(
                    this.x + rX(0.5, 0.5, this.r) - 0.5 + i,
                    this.y + rY(0.5, 0.5, this.r) - 0.5 + t,
                    s
                  )
                ) {
                  return true
                }
                if (
                  this.coll(
                    this.x + rX(0.5, -0.5, this.r) - 0.5 + i,
                    this.y + rY(0.5, -0.5, this.r) - 0.5 + t,
                    s
                  )
                ) {
                  return true
                }
              } else {
                if ('teal' == this.color) {
                  if (
                    this.coll(
                      this.x + rX(-1.5, -0.5, this.r) - 0.5 + i,
                      this.y + rY(-1.5, -0.5, this.r) - 0.5 + t,
                      s
                    )
                  ) {
                    return true
                  }
                  if (
                    this.coll(
                      this.x + rX(-0.5, -0.5, this.r) - 0.5 + i,
                      this.y + rY(-0.5, -0.5, this.r) - 0.5 + t,
                      s
                    )
                  ) {
                    return true
                  }
                  if (
                    this.coll(
                      this.x + rX(0.5, -0.5, this.r) - 0.5 + i,
                      this.y + rY(0.5, -0.5, this.r) - 0.5 + t,
                      s
                    )
                  ) {
                    return true
                  }
                  if (
                    this.coll(
                      this.x + rX(1.5, -0.5, this.r) - 0.5 + i,
                      this.y + rY(1.5, -0.5, this.r) - 0.5 + t,
                      s
                    )
                  ) {
                    return true
                  }
                } else {
                  if ('purple' == this.color) {
                    if (
                      this.coll(
                        this.x + rX(-1, 0, this.r) + i,
                        this.y + rY(-1, 0, this.r) + t,
                        s
                      )
                    ) {
                      return true
                    }
                    if (
                      this.coll(
                        this.x + rX(1, 0, this.r) + i,
                        this.y + rY(1, 0, this.r) + t,
                        s
                      )
                    ) {
                      return true
                    }
                    if (
                      this.coll(
                        this.x + rX(0, 0, this.r) + i,
                        this.y + rY(0, 0, this.r) + t,
                        s
                      )
                    ) {
                      return true
                    }
                    if (
                      this.coll(
                        this.x + rX(0, -1, this.r) + i,
                        this.y + rY(0, -1, this.r) + t,
                        s
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
    setBlock(i) {
      'blue' == this.color
        ? ((i[this.x + rX(-1, -1, this.r)][this.y + rY(-1, -1, this.r)] =
            new Block(
              this.x + rX(-1, -1, this.r),
              this.y + rY(-1, -1, this.r),
              this.color
            )),
          (i[this.x + rX(-1, 0, this.r)][this.y + rY(-1, 0, this.r)] = new Block(
            this.x + rX(-1, 0, this.r),
            this.y + rY(-1, 0, this.r),
            this.color
          )),
          (i[this.x][this.y] = new Block(this.x, this.y, this.color)),
          (i[this.x + rX(1, 0, this.r)][this.y + rY(1, 0, this.r)] = new Block(
            this.x + rX(1, 0, this.r),
            this.y + rY(1, 0, this.r),
            this.color
          )))
        : 'orange' == this.color
        ? ((i[this.x + rX(1, -1, this.r)][this.y + rY(1, -1, this.r)] = new Block(
            this.x + rX(1, -1, this.r),
            this.y + rY(1, -1, this.r),
            this.color
          )),
          (i[this.x + rX(-1, 0, this.r)][this.y + rY(-1, 0, this.r)] = new Block(
            this.x + rX(-1, 0, this.r),
            this.y + rY(-1, 0, this.r),
            this.color
          )),
          (i[this.x][this.y] = new Block(this.x, this.y, this.color)),
          (i[this.x + rX(1, 0, this.r)][this.y + rY(1, 0, this.r)] = new Block(
            this.x + rX(1, 0, this.r),
            this.y + rY(1, 0, this.r),
            this.color
          )))
        : 'green' == this.color
        ? ((i[this.x + rX(1, -1, this.r)][this.y + rY(1, -1, this.r)] = new Block(
            this.x + rX(1, -1, this.r),
            this.y + rY(1, -1, this.r),
            this.color
          )),
          (i[this.x + rX(-1, 0, this.r)][this.y + rY(-1, 0, this.r)] = new Block(
            this.x + rX(-1, 0, this.r),
            this.y + rY(-1, 0, this.r),
            this.color
          )),
          (i[this.x][this.y] = new Block(this.x, this.y, this.color)),
          (i[this.x + rX(0, -1, this.r)][this.y + rY(0, -1, this.r)] = new Block(
            this.x + rX(0, -1, this.r),
            this.y + rY(0, -1, this.r),
            this.color
          )))
        : 'red' == this.color
        ? ((i[this.x + rX(-1, -1, this.r)][this.y + rY(-1, -1, this.r)] =
            new Block(
              this.x + rX(-1, -1, this.r),
              this.y + rY(-1, -1, this.r),
              this.color
            )),
          (i[this.x + rX(1, 0, this.r)][this.y + rY(1, 0, this.r)] = new Block(
            this.x + rX(1, 0, this.r),
            this.y + rY(1, 0, this.r),
            this.color
          )),
          (i[this.x][this.y] = new Block(this.x, this.y, this.color)),
          (i[this.x + rX(0, -1, this.r)][this.y + rY(0, -1, this.r)] = new Block(
            this.x + rX(0, -1, this.r),
            this.y + rY(0, -1, this.r),
            this.color
          )))
        : 'yellow' == this.color
        ? ((i[this.x + rX(-0.5, -0.5, this.r) - 0.5][
            this.y + rY(-0.5, -0.5, this.r) - 0.5
          ] = new Block(
            this.x + rX(-0.5, -0.5, this.r) - 0.5,
            this.y + rY(-0.5, -0.5, this.r) - 0.5,
            this.color
          )),
          (i[this.x + rX(-0.5, 0.5, this.r) - 0.5][
            this.y + rY(-0.5, 0.5, this.r) - 0.5
          ] = new Block(
            this.x + rX(-0.5, 0.5, this.r) - 0.5,
            this.y + rY(-0.5, 0.5, this.r) - 0.5,
            this.color
          )),
          (i[this.x + rX(0.5, 0.5, this.r) - 0.5][
            this.y + rY(0.5, 0.5, this.r) - 0.5
          ] = new Block(
            this.x + rX(0.5, 0.5, this.r) - 0.5,
            this.y + rY(0.5, 0.5, this.r) - 0.5,
            this.color
          )),
          (i[this.x + rX(0.5, -0.5, this.r) - 0.5][
            this.y + rY(0.5, -0.5, this.r) - 0.5
          ] = new Block(
            this.x + rX(0.5, -0.5, this.r) - 0.5,
            this.y + rY(0.5, -0.5, this.r) - 0.5,
            this.color
          )))
        : 'teal' == this.color
        ? ((i[this.x + rX(-1.5, -0.5, this.r) - 0.5][
            this.y + rY(-1.5, -0.5, this.r) - 0.5
          ] = new Block(
            this.x + rX(-1.5, -0.5, this.r) - 0.5,
            this.y + rY(-1.5, -0.5, this.r) - 0.5,
            this.color
          )),
          (i[this.x + rX(-0.5, -0.5, this.r) - 0.5][
            this.y + rY(-0.5, -0.5, this.r) - 0.5
          ] = new Block(
            this.x + rX(-0.5, -0.5, this.r) - 0.5,
            this.y + rY(-0.5, -0.5, this.r) - 0.5,
            this.color
          )),
          (i[this.x + rX(0.5, -0.5, this.r) - 0.5][
            this.y + rY(0.5, -0.5, this.r) - 0.5
          ] = new Block(
            this.x + rX(0.5, -0.5, this.r) - 0.5,
            this.y + rY(0.5, -0.5, this.r) - 0.5,
            this.color
          )),
          (i[this.x + rX(1.5, -0.5, this.r) - 0.5][
            this.y + rY(1.5, -0.5, this.r) - 0.5
          ] = new Block(
            this.x + rX(1.5, -0.5, this.r) - 0.5,
            this.y + rY(1.5, -0.5, this.r) - 0.5,
            this.color
          )))
        : 'purple' == this.color &&
          ((i[this.x + rX(-1, 0, this.r)][this.y + rY(-1, 0, this.r)] = new Block(
            this.x + rX(-1, 0, this.r),
            this.y + rY(-1, 0, this.r),
            this.color
          )),
          (i[this.x + rX(1, 0, this.r)][this.y + rY(1, 0, this.r)] = new Block(
            this.x + rX(1, 0, this.r),
            this.y + rY(1, 0, this.r),
            this.color
          )),
          (i[this.x][this.y] = new Block(this.x, this.y, this.color)),
          (i[this.x + rX(0, -1, this.r)][this.y + rY(0, -1, this.r)] = new Block(
            this.x + rX(0, -1, this.r),
            this.y + rY(0, -1, this.r),
            this.color
          )))
      blockId++
    }
    coll(i, t, s) {
      return i < 0 || 9 < i || 19 < t || !!s[i][t]
    }
  }
  