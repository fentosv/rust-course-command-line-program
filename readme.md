### For local file

##### Add task

```bash
cargo run -- -j xxxxx.json add "take the dog for a walk"
```

##### Remove task

```bash
cargo run -- -j test-journal.json done 2
```

##### List tasks

```bash
cargo run -- -j test-journal.json list
```

### Default file

Without `-j` param, the file will be created in `/home`

### Publish

```bash
cargo run --release
```
