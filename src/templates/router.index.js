const itemRouter = require("./itemRouter");

function router(app) {
  const routers = [
    { path: '/item', router: itemRouter }
  ];
}

module.exports = router;