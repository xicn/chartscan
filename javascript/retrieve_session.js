const puppeteer = require('puppeteer');
const fs = require('fs');
require('dotenv').config();

(async () => {
    const spotify_login = 'https://accounts.spotify.com/en/login';
    const browser = await puppeteer.launch();
    const page = await browser.newPage();

    await page.goto(spotify_login, { waitUntil: 'networkidle2' });

    // Login
    await page.type('#login-username', process.env.spotify_username, { delay: 100 });
    await page.type('#login-password', process.env.spotify_password, { delay: 100 });

    await page.click('#login-button');

    // Wait 
    await page.waitForNavigation();

    const cookies = await page.cookies()

    // Save cookies off-line at ./spotify-session.json
    fs.writeFile('spotify-session.json', JSON.stringify(cookies, null, 2), (err) => {
        if (err) throw err;
        console.log('Completed write of cookies!');
    });


    await browser.close();
})();

