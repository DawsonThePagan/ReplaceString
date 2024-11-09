
# ReplaceString

Replace a string in a file.  
Part of BatchExtensions [Trello](https://trello.com/b/4J5sT1MN/batchextensions)

## Return Codes

+ 0 = Ok replaced text correctly
+ 1 = Ok replaced text correctly. Failed to delete temp file
+ 2 = Text asked to change was not found
+ 3 = IO error
+ 4 = Arguments provided were incorrect

## Arguments

```BATCH
ReplaceString.exe /file <file> /from <text from> [/to <text to> | /rmline]  {/nocase}
```

+ /file = Required, file to edit
+ /from = Required, text to change from. See /nocase for case sensitivity
+ /to = Optional, text to change to. Mutually exclusive with 'rmline', only set one or the other
+ /rmline = Optional, remove any line containing from. Mutually exclusive with 'to', only set one or the other
+ /nocase = Optional, by default from in case sensitive, this disables that

## License

Developed by Bailey-Tyreese Dawson as part of BatchExtensions
Licensed under MIT License
