const puppeteer = require('puppeteer');

(async () => {
    let code = '';
    const template = 'm.insert({pairIndex}, "{asset}");\n';
    // const template = "UPDATE `copytrading`.`trade` SET `symbol`='{asset}' WHERE `symbol`='{pairIndex}';\n";

    const browser = await puppeteer.launch({
        args: ['--proxy-server=http://127.0.0.1:7890'],
    });

    const page = await browser.newPage();
    await page.goto('https://gains-network.gitbook.io/docs-home/gtrade-leveraged-trading/pair-list', {
        waitUntil: 'networkidle2',
        timeout: 30000,
    });

    const rows = await page.$$('tbody tr');
    for (let i = 0; i < rows.length; i++) {
        const cells = await rows[i].$$('td');
        if (cells.length > 3) {
            code = code + template.replace('{pairIndex}', await page.evaluate(cell => cell.innerText, cells[0]))
                .replace('{asset}', await page.evaluate(cell => cell.innerText, cells[2]));
        }
    }

    await browser.close();

    process.stdout.write(code);
})();
