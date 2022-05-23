from enum import Enum


class Regions(Enum):
    Global = "global"
    US = "us"
    UK = "gb"
    Australia = "au"
    Canada = "ca"

    France = "fr"
    Netherlands = "nl"
    Finland = "fi"
    Norway = "no"
    Denmark = "dk"
    Sweden = "se"
    Switzerland = "ch"
    Ireland = "ie"
    NewZealand = "nz"

    Malaysia = "my"
    Singapore = "sg"
    Japan = "jp"
    India = "in"
    Philippines = "ph"
    # Thailand = 'th'
    Indonesia = "id"

    Germany = "de"
    Italy = "it"
    Austria = "at"
    Belgium = "be"
    Hungary = "hu"

    Portugal = "pt"
    Spain = "es"
    Brazil = "br"
    Colombia = "co"
    Mexico = "mx"
    Argentina = "ar"
    Poland = "pl"

    Chile = "cl"
    Turkey = "tr"
    # SouthKorea = 'kr'


index = 0
for x in Regions:
    print('"', x.value, '"', sep='', end=" ")
    print("=>", end=" ")
    # print('"', x.value, '"', sep='', end=",\n")
    print('Regions::', str(x.value).upper(), sep='', end=",\n")
    index = index + 1
