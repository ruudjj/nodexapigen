const express = require("express");
const router = require('./router');

const app = express();

app.use(express.json());

router(app);

app.listen(process.env.PORT || 3000, () => {
  console.log("Server running...");
});