# Model

`model` has 2 main types, `pre-trained`, `custom`.
- `pre-trained` models have pre-trained vocab lists and some pre-defined tokenization rules. e.g. [`bert_base_uncased`](https://huggingface.co/google-bert/bert-base-uncased).
- `custom` models will maintain their own vocab mapping. You can build a custom model based on your own corpus easily.

## Builtin models

We provide some builtin models to use directly.

- [bert-base-uncased](https://huggingface.co/google-bert/bert-base-uncased)
- [wiki_tocken](https://huggingface.co/datasets/iohadrubin/wikitext-103-raw-v1)
- [gemma2b](https://huggingface.co/google/gemma-2b)
- [llmlingua2](https://huggingface.co/microsoft/llmlingua-2-xlm-roberta-large-meetingbank)

## Huggingface model

We support importing models using [Hugging Face](https://huggingface.co/) config. You can use the `create_huggingface_model` function to import a model.

```sql
\set content `wget -q -O - https://huggingface.co/google-bert/bert-base-uncased/resolve/main/tokenizer.json`
SELECT create_huggingface_model('bert_import', :'content');
```

## Lindera model

We support importing models using [lindera](https://github.com/lindera/lindera) config. You can use the `create_lindera_model` function to import a model.

```sql
SELECT tokenizer_catalog.create_lindera_model('lindera_ipadic', $$
[segmenter]
mode = "normal"
  [segmenter.dictionary]
  kind = "ipadic"
[[character_filters]]
kind = "unicode_normalize"
  [character_filters.args]
  kind = "nfkc"
[[character_filters]]
kind = "japanese_iteration_mark"
  [character_filters.args]
  normalize_kanji = true
  normalize_kana = true
[[character_filters]]
kind = "mapping"
[character_filters.args.mapping]
"リンデラ" = "Lindera"
[[token_filters]]
kind = "japanese_compound_word"
  [token_filters.args]
  kind = "ipadic"
  tags = [ "名詞,数", "名詞,接尾,助数詞" ]
  new_tag = "名詞,数"
[[token_filters]]
kind = "japanese_number"
  [token_filters.args]
  tags = [ "名詞,数" ]
[[token_filters]]
kind = "japanese_stop_tags"
  [token_filters.args]
  tags = [
  "接続詞",
  "助詞",
  "助詞,格助詞",
  "助詞,格助詞,一般",
  "助詞,格助詞,引用",
  "助詞,格助詞,連語",
  "助詞,係助詞",
  "助詞,副助詞",
  "助詞,間投助詞",
  "助詞,並立助詞",
  "助詞,終助詞",
  "助詞,副助詞／並立助詞／終助詞",
  "助詞,連体化",
  "助詞,副詞化",
  "助詞,特殊",
  "助動詞",
  "記号",
  "記号,一般",
  "記号,読点",
  "記号,句点",
  "記号,空白",
  "記号,括弧閉",
  "その他,間投",
  "フィラー",
  "非言語音"
]
[[token_filters]]
kind = "japanese_katakana_stem"
  [token_filters.args]
  min = 3
[[token_filters]]
kind = "remove_diacritical_mark"
  [token_filters.args]
  japanese = false
$$);
```

Lindera has some builtin models, but we don't embed them in the extension directly. If you want to use them, you may need to compile the extension from source and enable `lindera-ipadic`, `lindera-ipadic-neologd`, `lindera-unidic`, `lindera-ko-dic`, `lindera-cc-cedict` features. See [installation](01-installation.md) for more details.

```sh
cargo pgrx install --sudo --release --features "lindera-ipadic lindera-ipadic-neologd lindera-unidic lindera-ko-dic lindera-cc-cedict"
```

## Custom model

You can build a custom model based on your own corpus easily. We provide following functions to help you build a custom model.

- `create_custom_model`: Create a custom model only.
- `create_custom_model_tokenizer_and_trigger`: Create a custom model, tokenizer, and trigger function. It will automatically insert embeddings into the target column.

### Example

with trigger (convenient):

```sql
-- Create a table to store documents
CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    passage TEXT,
    embedding INT[]
);

-- Create a text analyzer to generate tokens that can be used to train the model
SELECT create_text_analyzer('text_analyzer1', $$
pre_tokenizer = "unicode_segmentation"
[[character_filters]]
to_lowercase = {}
[[character_filters]]
unicode_normalization = "nfkd"
[[token_filters]]
skip_non_alphanumeric = {}
[[token_filters]]
stopwords = "nltk_english"
[[token_filters]]
stemmer = "english_porter2"
$$);

-- Prepare the custom model, tokenizer, and trigger
SELECT create_custom_model_tokenizer_and_trigger(
    tokenizer_name => 'tokenizer1',
    model_name => 'model1',
    text_analyzer_name => 'text_analyzer1',
    table_name => 'documents',
    source_column => 'passage',
    target_column => 'embedding'
);

-- Now you can insert some data and embeddings will be generated automatically
INSERT INTO documents (passage) VALUES 
('PostgreSQL is a powerful, open-source object-relational database system. It has over 15 years of active development.'),
('Full-text search is a technique for searching in plain-text documents or textual database fields. PostgreSQL supports this with tsvector.'),
('BM25 is a ranking function used by search engines to estimate the relevance of documents to a given search query.'),
('PostgreSQL provides many advanced features like full-text search, window functions, and more.'),
('Search and ranking in databases are important in building effective information retrieval systems.'),
('The BM25 ranking algorithm is derived from the probabilistic retrieval framework.'),
('Full-text search indexes documents to allow fast text queries. PostgreSQL supports this through its GIN and GiST indexes.'),
('The PostgreSQL community is active and regularly improves the database system.'),
('Relational databases such as PostgreSQL can handle both structured and unstructured data.'),
('Effective search ranking algorithms, such as BM25, improve search results by understanding relevance.');
```

without trigger (flexible):

```sql
-- Create a table to store documents
CREATE TABLE documents (
    id SERIAL PRIMARY KEY,
    passage TEXT,
    embedding INT[]
);

-- Create a text analyzer to generate tokens that can be used to train the model
SELECT create_text_analyzer('text_analyzer1', $$
pre_tokenizer = "unicode_segmentation"
[[character_filters]]
to_lowercase = {}
[[character_filters]]
unicode_normalization = "nfkd"
[[token_filters]]
skip_non_alphanumeric = {}
[[token_filters]]
stopwords = "nltk_english"
[[token_filters]]
stemmer = "english_porter2"
$$);

-- Create a custom model to generate embeddings from the original passage
SELECT create_custom_model('model1', $$
table = 'documents'
column = 'passage'
text_analyzer = 'text_analyzer1'
$$);

-- Create a tokenizer to tokenize the passage, note that the model is decoupled with the tokenizer
SELECT create_tokenizer('tokenizer1', $$
text_analyzer = 'text_analyzer1'
model = 'model1'
$$);

-- Insert some data
INSERT INTO documents (passage) VALUES 
('PostgreSQL is a powerful, open-source object-relational database system. It has over 15 years of active development.'),
('Full-text search is a technique for searching in plain-text documents or textual database fields. PostgreSQL supports this with tsvector.'),
('BM25 is a ranking function used by search engines to estimate the relevance of documents to a given search query.'),
('PostgreSQL provides many advanced features like full-text search, window functions, and more.'),
('Search and ranking in databases are important in building effective information retrieval systems.'),
('The BM25 ranking algorithm is derived from the probabilistic retrieval framework.'),
('Full-text search indexes documents to allow fast text queries. PostgreSQL supports this through its GIN and GiST indexes.'),
('The PostgreSQL community is active and regularly improves the database system.'),
('Relational databases such as PostgreSQL can handle both structured and unstructured data.'),
('Effective search ranking algorithms, such as BM25, improve search results by understanding relevance.');

-- Update the embedding column
UPDATE documents SET embedding = tokenize(passage, 'tokenizer1');
```

## Preload model

For each connection, Postgresql will load the model at the first time you use it. This may cause a delay for the first query. You can use the `add_preload_model` function to preload the model at the server startup.

```sh
psql -c "SELECT add_preload_model('model1')"
# restart the PostgreSQL to take effects
sudo docker restart container_name         # for pg_tokenizer running with docker
sudo systemctl restart postgresql.service  # for pg_tokenizer running with systemd
```

The default preload model is `llmlingua2`. You can change it by using `add_preload_model`, `remove_preload_model` functions.

Note: The pre-trained model may take a lot of memory (100MB for gemma2b, 200MB for llmlingua2). If you have a lot of models, you should consider the memory usage when you preload the model.
