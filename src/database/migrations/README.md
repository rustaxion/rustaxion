Database migrations, they are a handy thing, sadly I am not using them for their intended purpose.

So I made a document to communicate how I write my migrations, in case anyone even cares :^)

All incrementally numbered migrations (e.g. `m001_account.rs`), are the final state of a table until the milestone of version 1.0 is hit.

Meaning, whenever a table is updated, the migrations will be updated to reflect that.

So during the development, sqlite is used to make prototyping faster.

Once the milestone of 1.0 is hit, any changes to the schema shall be done by dated migrations, e.g. `m_2024-06-XX_player_newPropery.rs`, changing the numbered migrations will be forbidden and enforced by CI (if I ever make one).

So please, do not update your server if you are using an alpha version, because it will probably corrupt the database.
