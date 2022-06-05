const htmlparser2 = require("htmlparser2");
const { getChildren, find, hasAttrib } = require("domutils");

// Find rank
function find_rank(entry) {
    // Span tag that has this class_name contains rank data
    const rank_class_name = "Type__TypeElement-goli3j-0 hgLxdb";
    let rank = find((elem) => hasAttrib(elem, "class") && elem.attribs.class === rank_class_name,
        getChildren(entry),
        true)[0];

    return String(getChildren(rank)[0].data).trim().replace(/\s\s+/g, ' ');
}

// Find artist
function find_artist(entry) {
    // Span tag that has this class_name contains artist data
    const artist_class_name = "styled__StyledHyperlink-sc-135veyd-25 bVVLJU";
    let artist = find((elem) => hasAttrib(elem, "class") && elem.attribs.class === artist_class_name,
        getChildren(entry),
        true)[0];
    // console.log(String(getChildren(artist)[0].data).trim().replace(/\s\s+/g, ' '));

    return String(getChildren(artist)[0].data).trim().replace(/\s\s+/g, ' ');
}

// Find title
function find_title(entry) {
    // Span tag that has this class_name contains title data
    const title_class_name = "styled__StyledTruncatedTitle-sc-135veyd-22 kKOJRc";
    let title = find((elem) => hasAttrib(elem, "class") && elem.attribs.class === title_class_name,
        getChildren(entry),
        true)[0];
    // console.log(String(getChildren(title)[0].data).trim().replace(/\s\s+/g, ' '));

    return String(getChildren(title)[0].data).trim().replace(/\s\s+/g, ' ');

}

// Find streams
function find_streams(entry) {
    // Span tag that has this class_name contains streams data
    const streams_class_name = "TableCell__TableCellElement-sc-1nn7cfv-0 kJgiFu styled__RightTableCell-sc-135veyd-4 kGfYTK";
    let streams = find((elem) => hasAttrib(elem, "class") && elem.attribs.class === streams_class_name,
        getChildren(entry),
        true)[0];
    // console.log(String(getChildren(streams)[0].data).trim().replace(/\s\s+/g, ' '));

    return String(getChildren(streams)[0].data).trim().replace(/\s\s+/g, ' ');
}


module.exports = (htmlString) => {
    const table = getChildren(htmlparser2.parseDocument(htmlString))[0];
    const tbody = getChildren(table)[2];

    const chart_entries = find(
        (elem) => elem.name === "tr",
        getChildren(tbody),
        true
    );

    return chart_entries.map((elem) => {
        return {
            rank: find_rank(elem),
            title: find_title(elem),
            artist: find_artist(elem),
            streams: find_streams(elem)
        }
    });
};