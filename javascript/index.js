const puppeteer = require('puppeteer');
const bluebird = require('bluebird');
const fs = require('fs');
const parse = require('./parse');
const regions = require("./regions");

// Get session cookies
const cookiesString = fs.readFileSync('./spotify-session.json', 'utf8');
const cookies = JSON.parse(cookiesString);

// Parallelization constructs
const withBrowser = async (fn) => {
    const options = { headless: true };
    const browser = await puppeteer.launch(options);
    const page = await browser.newPage();

    // Setting session cookies
    await page.setCookie(...cookies);

    try {
        return await fn(browser);
    } finally {
        await browser.close();
    }
}

const withPage = (browser) => async (fn) => {
    const page = await browser.newPage();
    try {
        return await fn(page);
    } finally {
        await page.close();
    }
}

(async () => {
    // const urls = generate_urls('2022-06-04');
    const urls = ['https://charts.spotify.com/charts/view/regional-global-daily/latest'];

    const results = await withBrowser(async (browser) => {
        return bluebird.map(urls, async (url) => {
            return withPage(browser)(async (page) => {
                await page.goto(url, { waitUntil: 'networkidle2' });

                const data = await page.evaluate(() => document.getElementsByClassName('Table__TableElement-evwssh-0 jaKCLL styled__StyledTable-sc-135veyd-7 QMWIc')[0].outerHTML);
                return parse(String(data));

            }).then((r) => ({ result: r, url: url }), (e) => ({ error: e, url: url }));
        }, { concurrency: 3 });
    });
    console.log(results);

})();


let single_page = async () => {
    await withBrowser(async (browser) => {
        const result = await withPage(browser)(async (page) => {
            await page.goto('https://charts.spotify.com/charts/view/regional-global-daily/latest', { waitUntil: 'networkidle2' });
            const outerHtml = await page.evaluate(() => document.getElementsByClassName('Table__TableElement-evwssh-0 jaKCLL styled__StyledTable-sc-135veyd-7 QMWIc')[0].outerHTML);
            const data = parse(String(outerHtml));
            return data;
        });
        console.log(result);
    });
};

function retrieve_url(code, date) {
    return `https://charts.spotify.com/charts/view/regional-${code}-daily/${date}`
}

function generate_urls(date) {
    let result = [];
    for (const region in regions) {
        let code = regions[region];
        result.push(retrieve_url(code, date));
    }
    return result;
}