initSidebarItems({"fn":[["get_clipboard","Retrieves clipboard content."],["get_clipboard_formats","Extracts available clipboard formats."],["get_clipboard_string","Retrieves clipboard content in UTF16 format and convert it to String."],["get_format_name","Returns format name based on it's code."],["set_clipboard","Set clipboard with text."]],"mod":[["clipboard_formats","Standart clipboard formats."],["wrapper","Provides direct wrappers to WinAPI functions."]],"struct":[["ClipboardManager","Clipboard manager provides a primitive hack for console application to handle updates of clipboard. It uses ```get_clipboard_seq_num``` to determines whatever clipboard is updated or not. Due to that this manager is a bit hacky and not exactly right way to properly work with clipboard. On other hand it is the best and most easy option for console application as a proper window is required to be created to work with clipboard."],["WindowsError","Represents Windows error code."]]});