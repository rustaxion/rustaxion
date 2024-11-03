## Migrations

Please read the dedicated README for the migrations under the migrations directory.

## Entities

Entities are auto-generated using sea-orm-cli, these are the steps on modifying the schema:

1. Create a migration with ur changes
2. Build and run the code
3. Run `sea-orm-cli generate entity -u 'postgres://postgres:postgres@localhost:5432/postgres' -o src/database/entities` from the root of the project

You might notice that this restricts what changes you can do, since the code has to compile.
This means that you must modify the schema first, update the entities and then proceed to update the code that uses the entities.

Otherwise you will get compile-time errors.
