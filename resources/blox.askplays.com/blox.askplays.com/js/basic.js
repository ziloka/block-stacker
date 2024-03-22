window.addEventListener('DOMContentLoaded', () => {
    var o = localStorage.getItem('username')
    const t = document.getElementById('profile-dropdown')
    if (null != o && !document.getElementById('profile-page') && t) {
      const n = document.getElementById('account-menu')
      t.style.display = 'block'
      let e = document.createElement('a')
      e.className = 'top'
      e.id = 'profile-page'
      e.innerText = 'Profile'
      e.href = '/profile?player=' + o
      n.parentElement.insertBefore(e, n)
      n.remove()
      document.getElementById('times-page').href = document
        .getElementById('times-page')
        .href.replace(/(.*times).*/, '$1?player=' + o)
    }
    o = JSON.parse(localStorage.getItem('settings'))
    null != o && o.theme && (document.body.parentNode.className = o.theme)
    document.getElementById('log-out') &&
      (document.getElementById('log-out').addEventListener('click', (e) => {
        document.cookie = 'username=; Max-Age=-99999999;'
        document.cookie = 'password=; Max-Age=-99999999;'
        localStorage.removeItem('username')
        localStorage.removeItem('password')
        localStorage.removeItem('sessionID')
        document.location.reload()
      }),
      document.getElementById('log-out').addEventListener('keydown', (e) => {
        'Enter' == e.code &&
          ((document.cookie = 'username=; Max-Age=-99999999;'),
          (document.cookie = 'password=; Max-Age=-99999999;'),
          localStorage.removeItem('username'),
          localStorage.removeItem('password'),
          localStorage.removeItem('sessionID'),
          document.location.reload())
      }))
  })
  