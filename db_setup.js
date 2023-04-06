#!/usr/bin/arangosh --javascript.execute

var db = require('internal').db;

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

var emailIndexExists = db.User.getIndexes().some(
  (index) =>
    index.type === 'persistent' &&
    index.unique === true &&
    index.sparse === false &&
    index.fields.includes('email')
);

if (!emailIndexExists) {
  db.User.ensureIndex({
    type: 'persistent',
    unique: true,
    sparse: false,
    fields: ['email'],
  });
}
