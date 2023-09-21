librime-rpc
---

A simple JSON-RPC wrapper for librime.

## Usage:
<pre>JSON-RPC server for librime

<u style="text-decoration-style:single"><b>Usage:</b></u> <b>librime-rpc</b> [OPTIONS] &lt;USER_DATA_DIR&gt; &lt;SHARED_DATA_DIR&gt;

<u style="text-decoration-style:single"><b>Arguments:</b></u>
  &lt;USER_DATA_DIR&gt;    librime user data directory
  &lt;SHARED_DATA_DIR&gt;  librime shared data directory

<u style="text-decoration-style:single"><b>Options:</b></u>
  <b>-p</b>, <b>--port</b> &lt;PORT&gt;  Port the server listens on [default: 8000]
  <b>-h</b>, <b>--help</b>         Print help</pre>

```console
~ ❯ curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"simulate_key_sequence","params":"nihao"}' \
     http://localhost:8001
{"jsonrpc":"2.0","result":{"commit":null,"composition":{"cursor_pos":6,"length":6,"preedit":"ni hao","sel_end":6,"sel_start":0},"menu":{"candidates":[{"comment":null,"text":"你好"},{"comment":null,"text":"妳好"},{"comment":null,"text":"逆號"},{"comment":null,"text":"你"},{"comment":null,"text":"擬"}],"highlighted_candidate_index":0,"is_last_page":false,"num_candidates":5,"page_no":0,"page_size":5,"select_keys":null},"select_labels":null,"status":{"is_ascii_mode":false,"is_ascii_punct":false,"is_composing":true,"is_disabled":false,"is_full_shape":false,"is_simplified":false,"is_traditional":false}},"id":"id"}%
~ ❯ curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"simulate_key_sequence","params":"{space}"}' \
     http://localhost:8001
{"jsonrpc":"2.0","result":{"commit":"你好","composition":{"cursor_pos":0,"length":0,"preedit":null,"sel_end":0,"sel_start":0},"menu":{"candidates":[],"highlighted_candidate_index":0,"is_last_page":false,"num_candidates":0,"page_no":0,"page_size":0,"select_keys":null},"select_labels":null,"status":{"is_ascii_mode":false,"is_ascii_punct":false,"is_composing":false,"is_disabled":false,"is_full_shape":false,"is_simplified":false,"is_traditional":false}},"id":"id"}%                                                                       
```
