!(function (e, t) {
    'function' == typeof define && define.amd
      ? define(['./libgif'], t)
      : 'object' == typeof exports
      ? (module.exports = t(require('./libgif')))
      : (e.RubbableGif = t(e.SuperGif))
  })(this, function (e) {
    return function (n) {
      function t() {
        function o(e) {
          return n.vp_l ? e - n.vp_l : e
        }
        var e = l.get_canvas(),
          a = n.vp_w || e.width,
          u = Math.floor(a / (2 * l.get_length())),
          i = 0,
          r = 0,
          t = 'ontouchend' in document,
          c = 0
        e.addEventListener(t ? 'touchstart' : 'mousedown', function (e) {
          e.preventDefault()
          l.get_auto_play() && l.pause()
          var t = e.touches && 0 < e.touches.length ? e.touches[0] : e,
            n = 0 < t.layerX ? o(t.layerX) : a / 2
          l.move_to(Math.floor((n / a) * (l.get_length() - 1)))
          r = e.timeStamp
          i = o(t.pageX)
        })
        e.addEventListener(t ? 'touchend' : 'mouseup', function (e) {
          i = r = 0
          l.get_auto_play() && l.play()
        })
        e.addEventListener(t ? 'touchmove' : 'mousemove', function (e) {
          e.preventDefault()
          var t = e.touches && 0 < e.touches.length ? e.touches[0] : e,
            n = o(t.pageX)
          currentDistance = 0 === i ? 0 : Math.abs(n - i)
          currentTime = e.timeStamp
          0 !== r &&
            currentDistance > u &&
            (n < i && 0 < l.get_current_frame() && l.move_relative(-1),
            i < n &&
              l.get_current_frame() < l.get_length() - 1 &&
              l.move_relative(1),
            (r = e.timeStamp),
            (i = o(t.pageX)))
          e.timeStamp
          c++
          document.getElementById('tickles' + ((c % 5) + 1)) &&
            document.getElementById('tickles' + ((c % 5) + 1)).play()
          e.timeStamp
        })
      }
      var l = new e(n)
      return (
        (l.orig_load = l.load),
        (l.load = function (e) {
          l.orig_load(function () {
            e && e()
            t()
          })
        }),
        l
      )
    }
  })
  