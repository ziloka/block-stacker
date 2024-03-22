let inMobileLayout = false
function mobileLayout() {
  if (((inMobileLayout = true), document.getElementById('container'))) {
    let i = {
      mobilePositions: [
        {
          left: '2vw',
          bottom: '22vw',
        },
        {},
        {
          left: '42vw',
          bottom: '22vw',
        },
        {
          right: '22vw',
          bottom: '22vw',
        },
        {
          right: '2vw',
          bottom: '22vw',
        },
        {
          left: '2vw',
          bottom: '2vw',
        },
        {
          left: '22vw',
          bottom: '2vw',
        },
        {
          left: '42vw',
          bottom: '2vw',
        },
        {
          right: '22vw',
          bottom: '2vw',
        },
        {
          right: '2vw',
          bottom: '2vw',
        },
      ],
      moreSpace: true,
    }
    if (
      ((i = Object.assign(i, JSON.parse(localStorage.getItem('settings')))),
      (document.getElementById('container').style.display = 'grid'),
      (document.getElementById('main').style.justifyContent = 'center'),
      document.getElementById('chat') &&
        (document
          .getElementById('aside')
          .appendChild(document.getElementById('chat')),
        (document.getElementById('chat').classList += ' mobile')),
      document.getElementById('mobile-buttons'))
    ) {
      var e
      document.getElementById('mobile-buttons').style.display = 'flex'
      document.getElementById('mobile-controls').classList.remove('hidden')
      document.getElementById('settings').style.left = '20px'
      document.getElementById('settings').style.right = '20px'
      document.getElementById('mobile-chat').style.display = 'block'
      i.moreSpace &&
        ((e =
          window.innerHeight -
          document.getElementById('main').getBoundingClientRect().height -
          document.getElementById('header').getBoundingClientRect().height),
        (document.getElementById('mobile-buttons').style.height = e + 'px'))
      for (
        let o = 0;
        o < document.getElementById('mobile-buttons').childElementCount;
        o++
      ) {
        let e = document.getElementById('mobile-buttons').children[o]
        'SPAN' === e.tagName &&
          (e.addEventListener(
            'touchstart',
            (e) => {
              var t
              document.activeElement.blur()
              mobileMove ||
                ((t = keyBinds[idToCode[e.currentTarget.id]]),
                keyDown({
                  code: t,
                  repeat: false,
                }))
              mobileMove &&
                -1 == mobileButton &&
                ((mobileButton = o),
                (mOffX =
                  e.touches[0].clientX -
                  e.currentTarget.getBoundingClientRect().x),
                (mOffY =
                  e.currentTarget.getBoundingClientRect().height -
                  (e.touches[0].clientY -
                    e.currentTarget.getBoundingClientRect().y)))
            },
            { passive: true }
          ),
          e.addEventListener('touchend', (e) => {
            e = keyBinds[idToCode[e.currentTarget.id]]
            keyUp({ code: e })
          }),
          e.addEventListener('click', (e) => {
            e.preventDefault()
          }),
          i.mobilePositions[o].left &&
            ((e.style.right = ''), (e.style.left = i.mobilePositions[o].left)),
          i.mobilePositions[o].right &&
            ((e.style.left = ''), (e.style.right = i.mobilePositions[o].right)),
          i.mobilePositions[o].bottom &&
            (e.style.bottom = i.mobilePositions[o].bottom))
      }
      document.body.addEventListener('touchmove', (t) => {
        if (mobileMove && -1 != mobileButton) {
          var o = ((t.touches[0].clientX - mOffX) / window.innerWidth) * 100,
            t =
              ((window.innerHeight - (t.touches[0].clientY + mOffY)) /
                window.innerWidth) *
              100
          let e =
            document.getElementById('mobile-buttons').children[mobileButton]
          e &&
            (o <= 42
              ? ((e.style.left = o + 'vw'), (e.style.right = ''))
              : ((e.style.right = 84 - o + 'vw'), (e.style.left = '')),
            (e.style.bottom = t + 'vw'))
        }
      })
      document.body.addEventListener('touchend', (e) => {
        if (mobileMove && -1 != mobileButton) {
          var t =
            document.getElementById('mobile-buttons').children[mobileButton]
          if (t) {
            let e = { bottom: t.style.bottom }
            t.style.left && (e.left = t.style.left)
            t.style.right && (e.right = t.style.right)
            i.mobilePositions[mobileButton] = e
            setCookie('settings', JSON.stringify(i))
          }
          mobileButton = -1
        }
      })
    }
  }
  document
    .getElementById('other-dropdown')
    .appendChild(document.getElementById('leaderboard-page'))
  document
    .getElementById('other-dropdown')
    .appendChild(document.getElementById('map-maker'))
  document.getElementById('about-page') &&
    document
      .getElementById('other-dropdown')
      .appendChild(document.getElementById('about-page'))
  document
    .getElementById('other-dropdown')
    .appendChild(document.getElementById('discord-page'))
}
const mobileCheck = function () {
  if (navigator.userAgentData) {
    return navigator.userAgentData.mobile
  }
  let e = false
  var t
  return (
    (t = navigator.userAgent || navigator.vendor || window.opera),
    (e =
      /(android|bb\d+|meego).+mobile|avantgo|bada\/|blackberry|blazer|compal|elaine|fennec|hiptop|iemobile|ip(hone|od)|iris|kindle|lge |maemo|midp|mmp|mobile.+firefox|netfront|opera m(ob|in)i|palm( os)?|phone|p(ixi|re)\/|plucker|pocket|psp|series(4|6)0|symbian|treo|up\.(browser|link)|vodafone|wap|windows ce|xda|xiino/i.test(
        t
      ) ||
      /1207|6310|6590|3gso|4thp|50[1-6]i|770s|802s|a wa|abac|ac(er|oo|s\-)|ai(ko|rn)|al(av|ca|co)|amoi|an(ex|ny|yw)|aptu|ar(ch|go)|as(te|us)|attw|au(di|\-m|r |s )|avan|be(ck|ll|nq)|bi(lb|rd)|bl(ac|az)|br(e|v)w|bumb|bw\-(n|u)|c55\/|capi|ccwa|cdm\-|cell|chtm|cldc|cmd\-|co(mp|nd)|craw|da(it|ll|ng)|dbte|dc\-s|devi|dica|dmob|do(c|p)o|ds(12|\-d)|el(49|ai)|em(l2|ul)|er(ic|k0)|esl8|ez([4-7]0|os|wa|ze)|fetc|fly(\-|_)|g1 u|g560|gene|gf\-5|g\-mo|go(\.w|od)|gr(ad|un)|haie|hcit|hd\-(m|p|t)|hei\-|hi(pt|ta)|hp( i|ip)|hs\-c|ht(c(\-| |_|a|g|p|s|t)|tp)|hu(aw|tc)|i\-(20|go|ma)|i230|iac( |\-|\/)|ibro|idea|ig01|ikom|im1k|inno|ipaq|iris|ja(t|v)a|jbro|jemu|jigs|kddi|keji|kgt( |\/)|klon|kpt |kwc\-|kyo(c|k)|le(no|xi)|lg( g|\/(k|l|u)|50|54|\-[a-w])|libw|lynx|m1\-w|m3ga|m50\/|ma(te|ui|xo)|mc(01|21|ca)|m\-cr|me(rc|ri)|mi(o8|oa|ts)|mmef|mo(01|02|bi|de|do|t(\-| |o|v)|zz)|mt(50|p1|v )|mwbp|mywa|n10[0-2]|n20[2-3]|n30(0|2)|n50(0|2|5)|n7(0(0|1)|10)|ne((c|m)\-|on|tf|wf|wg|wt)|nok(6|i)|nzph|o2im|op(ti|wv)|oran|owg1|p800|pan(a|d|t)|pdxg|pg(13|\-([1-8]|c))|phil|pire|pl(ay|uc)|pn\-2|po(ck|rt|se)|prox|psio|pt\-g|qa\-a|qc(07|12|21|32|60|\-[2-7]|i\-)|qtek|r380|r600|raks|rim9|ro(ve|zo)|s55\/|sa(ge|ma|mm|ms|ny|va)|sc(01|h\-|oo|p\-)|sdk\/|se(c(\-|0|1)|47|mc|nd|ri)|sgh\-|shar|sie(\-|m)|sk\-0|sl(45|id)|sm(al|ar|b3|it|t5)|so(ft|ny)|sp(01|h\-|v\-|v )|sy(01|mb)|t2(18|50)|t6(00|10|18)|ta(gt|lk)|tcl\-|tdg\-|tel(i|m)|tim\-|t\-mo|to(pl|sh)|ts(70|m\-|m3|m5)|tx\-9|up(\.b|g1|si)|utst|v400|v750|veri|vi(rg|te)|vk(40|5[0-3]|\-v)|vm40|voda|vulc|vx(52|53|60|61|70|80|81|83|85|98)|w3c(\-| )|webc|whit|wi(g |nc|nw)|wmlb|wonu|x700|yas\-|your|zeto|zte\-/i.test(
        t.substr(0, 4)
      )
        ? true
        : e)
  )
}
mobileCheck() && !inMobileLayout && mobileLayout()
