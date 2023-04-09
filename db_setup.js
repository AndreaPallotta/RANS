#!/usr/bin/arangosh --javascript.execute

var db = require('internal').db;

const createIndex = ({ collection, type, unique, sparse, field }) => {
  var indexExists = db[collection]
    .getIndexes()
    .some(
      (index) =>
        index.type === type &&
        index.unique === unique &&
        index.sparse === sparse &&
        index.fields.includes(field)
    );

  if (indexExists) {
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

var collectionsToCreate = ['User', 'Item'];

collectionsToCreate.forEach((name) => {
  if (!collectionsNames.includes(name)) {
    db._create(name);
  }
});

createIndex('User', 'persistent', true, false, 'email');
createIndex('Item', 'persistent', true, false, 'name');
