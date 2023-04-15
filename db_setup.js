#!/usr/bin/arangosh --javascript.execute

var db = require('internal').db;

const schemas = {
  User: {
    rule: {
      properties: {
        first_name: { type : "string" },
        last_name: { type: "string" },
        email: { type: "string" },
        password: { type: "string" },
        role: { enum: ["CUSTOMER", "VENDOR"] }
      },
      additionalProperties: false,
      required: ["first_name", "last_name", "email", "password", "role"],
    },
    level: "moderate",
    message: "One or more user properties are missing or malformatted",
  },
  Order: {
    rule: {
      properties: {
        date: { type : "string" },
        user_id: { type: "string" },
        item_id: { type: "string" },
        item_name: { type: "string" },
        quantity: { type: "integer" },
        price: { type: "number" },
      },
      additionalProperties: false,
      required: ["date", "user_id", "item_id", "item_name", "quantity", "price"],
    },
    level: "moderate",
    message: "One or more order properties are missing or malformatted",
  },
  Item: {
    rule: {
      properties: {
        name: { type : "string" },
        description: { type: "string" },
        quantity: { type: "integer" },
        price: { type: "number" },
      },
      additionalProperties: false,
      required: ["name", "description", "quantity", "price"],
    },
    level: "moderate",
    message: "One or more item properties are missing or malformatted",
  }
}

const createIndex = (collection, type, unique, sparse, field) => {
  var indexExists = db[collection]
    .getIndexes()
    .some(
      (index) =>
        index.type === type &&
        index.unique === unique &&
        index.sparse === sparse &&
        index.fields.includes(field)
    );

  if (!indexExists) {
    db[collection].ensureIndex({
      type,
      unique,
      sparse,
      fields: [field],
    });
  }
}

var dbs = db._databases();

if (!dbs.includes('project2')) {
  db._createDatabase('project2');
}

db._useDatabase('project2');

var collections = db._collections();
var collectionsNames = collections.reduce(
  (acc, collection) => [...acc, collection.name()],
  []
);

var collectionsToCreate = ['User', 'Item', 'Order'];

collectionsToCreate.forEach((name) => {
  if (!collectionsNames.includes(name)) {
    db._create(name, { schema: schemas[name]});
  }
});

createIndex('User', 'persistent', true, false, 'email');
createIndex('Item', 'persistent', true, false, 'name');
