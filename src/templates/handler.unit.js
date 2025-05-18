
const pool = require("../db");
const AppError = require("../utils/appError");

async function readAllItems() {
  const result = await pool.query(
    `
    SELECT * FROM item
    `,
    []
  );

  return result.rows;
}

async function readItem(itemId) {
  const result = await pool.query(
    `
    SELECT * FROM item
    WHERE id = $1
    `,
    [itemId]
  );

  if (result.rows.length === 0) {
    throw new AppError("item not found", 404);
  }

  return result.rows[0];
}

async function createItem(name, description, muscle_group, equipment_type) {
  const result = await pool.query(
    `INSERT INTO item (name, description, muscle_group, equipment_type)
     VALUES ($1, $2, $3, $4, $5)
     RETURNING *`,
    [name, description, muscle_group, equipment_type]
  );
  return result.rows[0];
}

async function updateItem(itemId, name, description) {
  const result = await pool.query(
    `UPDATE item
     SET name = $2, description = $3
     WHERE id = $1
     RETURNING *`,
    [itemId, name, description]
  );

  if (result.rows.length === 0) {
    throw new AppError("item not found", 404);
  }

  return result.rows[0];
}

async function deleteItem(itemId) {
  const result = await pool.query(
    `DELETE FROM item
     WHERE id = $1
     RETURNING *`,
    [itemId, userId]
  );

  if (result.rows.length === 0) {
    throw new AppError("item not found", 404);
  }

  return result.rows[0];
}


module.exports = {
    createItem,
    readItem,
    updateItem,
    deleteItem,
    readAllItems,
};
