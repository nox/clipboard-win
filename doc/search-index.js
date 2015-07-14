var searchIndex = {};
searchIndex['clipboard_win'] = {"items":[[0,"","clipboard_win","Clipboard WIN API",null,null],[3,"ClipboardManager","","Clipboard manager provides a primitive hack for console application to handle updates of\nclipboard. It uses ```get_clipboard_seq_num``` to determines whatever clipboard is updated or\nnot. Due to that this manager is a bit hacky and not exactly right way to properly work with\nclipboard. On other hand it is the best and most easy option for console application as a proper\nwindow is required to be created to work with clipboard.",null,null],[5,"get_clipboard_seq_num","","Wrapper around ```GetClipboardSequenceNumber```.",null,{"inputs":[],"output":{"name":"option"}}],[5,"set_clipboard","","Set clipboard with text.",null,{"inputs":[{"name":"t"}],"output":{"name":"result"}}],[5,"rust_strlen","","Rust variant of strlen.",null,null],[5,"get_clipboard","","Extracts clipboard content and convert it to String.",null,{"inputs":[],"output":{"name":"result"}}],[5,"get_clipboard_formats","","Extracts available clipboard formats.",null,{"inputs":[],"output":{"name":"result"}}],[5,"get_format_name","","Returns format name based on it's code.",null,{"inputs":[{"name":"u32"}],"output":{"name":"option"}}],[11,"new","","Construct new ClipboardManager with default settings",0,{"inputs":[{"name":"clipboardmanager"}],"output":{"name":"clipboardmanager"}}],[11,"delay","","Configure manager's delay between accessing clipboard.",0,{"inputs":[{"name":"clipboardmanager"},{"name":"u32"}],"output":{"name":"clipboardmanager"}}],[11,"ok_callback","","Sets callback for successfully retrieved clipboard's text.",0,null],[11,"err_callback","","Sets callback for failed retrieval of clipboard's text.",0,null],[11,"run","","Starts manager loop.",0,null]],"paths":[[3,"ClipboardManager"]]};
initSearch(searchIndex);
