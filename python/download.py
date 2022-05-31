import os
from os import path
import bs4
import threading
import time
import pandas
import cloudscraper
import sys


from regions import Regions


def get_location(region):
    return "/home/ubuntu/project/chartscan/SpotifyData/" + region


def range_date(start_date, end_date):
    mydates = pandas.date_range(start_date, end_date).tolist()
    list_1 = list()
    for date in mydates:
        list_1.append(pandas.Timestamp(date).date().strftime("%Y-%m-%d"))
    return list_1


def get_info(tag_obj, str, testing=False):
    if str == "chart-table-track":
        return get_track_artist(tag_obj, testing)

    info = tag_obj["class"][0]
    if info == str:
        return tag_obj.text
    else:
        if testing is True:
            return "This is not a " + str + " tag"


def get_position(tag_obj, testing=False):
    if tag_obj["class"][0] == "chart-table-position":
        return "chart-table-position" + " is : " + tag_obj.text
    else:
        if testing is True:
            return "This is not a " + "chart-table-position" + " tag"


def get_track_artist(tag_obj, testing=False):
    if tag_obj["class"][0] == "chart-table-track":
        tag = list(tag_obj.children)
        track_name = tag[1].text
        artist = tag[3].text[3:]
        return track_name, artist
    else:
        if testing is True:
            return "This is not a " + "chart-table-track" + " tag"


def get_streams(tag_obj, testing=False):
    if tag_obj["class"][0] == "chart-table-streams":
        return "chart-table-streams" + " is : " + tag_obj.text
    else:
        if testing is True:
            return "This is not a " + "chart-table-streams" + " tag"

# Get 201 rows that contains the tracks + header information


def get_tracks(bs_obj, optional=False):
    list_return = bs_obj.find_all("tr")
    if optional is True:
        return list(list_return)[1:]
    return list(list_return)


def write_file(out_put, list_tracks):
    with open(out_put, "w") as f:
        for track_tag in list_tracks:
            pos = ""
            stream = ""
            track = ""
            art = ""
            for info in track_tag:
                if isinstance(info, bs4.element.NavigableString):
                    # print('yes it is instant of that')
                    continue
                elif isinstance(info, bs4.element.Tag):
                    str = info["class"][0]
                    if str == "chart-table-position":
                        pos = get_info(info, str)
                    elif str == "chart-table-track":
                        if info is not None:
                            (track, art) = get_track_artist(
                                info)  # type: ignore
                    elif str == "chart-table-streams":
                        stream = get_info(info, str)
            if "*" in track:
                track = "\"" + track + "\""
            if "*" in art:
                art = "\"" + art + "\""
            f.write("{}*{}*{}*{}\n".format(pos, track, art, stream))

# This one also bypasses cloudflare


def make_soup(region_code, date):
    url_link = "https://spotifycharts.com/regional/" + region_code + "/daily/" + date
    scraper = cloudscraper.create_scraper()
    content = scraper.get(url_link).text
    soup = bs4.BeautifulSoup(content, "html.parser")
    return soup


def get_download(date, region, region_code):
    soup = make_soup(region_code, date)
    lis = get_tracks(soup, True)
    if len(lis) != 200:
        print('get_download("{}", "{}", "{}")'.format(
            date, region, region_code), file=sys.stderr)
        return print("[Error]: in html file", region, region_code, date)

    location_dir = get_location(region_code)
    if path.exists(location_dir) is False:
        os.makedirs(location_dir)

    out_put = location_dir + "/" + date + ".csv"
    write_file(out_put, lis)
    print("[Success]:", region, region_code, date)


def createThread(date, region, region_code):
    download_thread = threading.Thread(
        target=get_download, args=(date, region, region_code)
    )
    download_thread.start()
    return download_thread


def download_all_regions(date):
    threads = []
    for countries in Regions:
        threads.append(createThread(
            date, countries.name, countries.value))

    for thread in threads:
        thread.join()


start_time = time.time()

if __name__ == "__main__":
    download_all_regions("2022-05-30")
