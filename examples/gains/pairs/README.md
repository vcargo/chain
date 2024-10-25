# Pairs

Get Gains Network Pairs

```
node pairs-extractor.js
```

---

Open [Pair List](https://gains-network.gitbook.io/docs-home/gtrade-leveraged-trading/pair-list) page with Chrome and Run following code in Console:

```
let code = '';
const template = 'm.insert({pairIndex}, "{asset}");\n';
const rows = document.querySelectorAll("tbody tr");
rows.forEach((row) => {
    const cells = row.querySelectorAll("td");
    if (cells.length >= 3) {
      code = code + template.replace('{pairIndex}', cells[0].textContent).replace('{asset}', cells[2].textContent);
    }
});
console.log(code);
```
