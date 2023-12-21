# CodeDescriber

## 簡介
CodeDescriber 是一款專門用於搜尋和整理程式碼註釋的命令行工具。它可以在指定的資料夾中搜尋特定擴展名的檔案，從程式碼中提取被兩個@符號包圍的說明文字，並將這些信息連同其文件路徑一起輸出到Excel檔案中。這個工具特別適用於博客來內部進行程式碼的盤點和稽核。

## 功能
- 在指定資料夾中搜尋特定擴展名的檔案。
- 支持遞迴搜尋子資料夾。
- 從程式碼中提取特定格式的註解說明。
- 將提取的註解和檔案路徑輸出到Excel檔案。

## 安裝
將可執行檔放到您指定的位置後依據選項使用

## 使用方法
```bash
code_describer [OPTIONS] --extentions <EXTENTIONS>
```

## 選項
```bash
`-p, --path <PATH>`: 指定要搜尋的資料夾路徑。
`-r, --recursive`: 在給定目錄中遞迴搜尋。
`-e, --extentions <EXTENTIONS>`: 指定要搜尋的檔案擴展名，例如：swift,js,py。
`-o, --output <OUTPUT>`: 指定輸出結果的檔案。
`-h, --help`: 顯示幫助信息。
`-V, --version`: 顯示版本信息。
```

## 範例
```bash
code_describer -p ~/Documents/Books/ebooks -r -e swift,js -o ~/Documents/Books/Output

遞迴搜尋~/Documents/Books/ebooks資料夾內副檔名為swift以及js的檔  
將excel檔案輸出到~/Documents/Books/Output 預設檔案名稱為result.xlsx
```

