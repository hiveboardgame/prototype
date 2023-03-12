# prototype
Hive prototype, lihive meets rust backend

## Requirements

* rust/cargo
* nodejs
* a Firebase project w/ Authentication setup
    * Sign-in providers enabled:
        * Google
        * Anonymous
* a Postgres database

## Running locally

1. Setup a Firebase project as defined above, making note of its Web API key, project ID, and project auth domain
2. Create an `.env.local` file for the frontend:

`js/frontend/.env.local`
```
NEXT_PUBLIC_FIREBASE_API_KEY=
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=
NEXT_PUBLIC_FIREBASE_PROJECT_ID="
```

3. Build the static frontend files:

```
$ npm install
$ npm run --workspace js/hive-lib build
$ npm run --workspace js/frontend build
```

4. Create a `.env` file for the backend:

`backend/.env`
```
DATABASE_URL=              # whatever the connection string for your Postgres database is
FIREBASE_JWT_ISSUER=       # this should be "https://securetoken.google.com/<projectId>", where projectId is your firebase project ID as above
STATIC_FILES_PATH=./dist   # we'll set this up in a moment
```

4. Run the Diesel migrations, if you haven't already:

```
$ cd backend
$ diesel migration run
```

5. Run the backend server:

```
$ cd backend
$ cargo run
```

## Previous works
[lihive frontend](https://github.com/atdyer/lihive) by [atdyer](https://github.com/atdyer/lihive)

[hive rust backend](https://github.com/klautcomputing/hive) by [klautcomputing](https://github.com/klautcomputing)
