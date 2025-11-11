# SzimatLauncher

## Mi ez?
Ez a hatalmas RP-seknek szól, akik Steames GTA:SA-jukba akarják pumpálni az éveket úgy, hogy ezzel nagyobb macera ne legyen.

## Mit csinál?
Ez a pár megás exe ha kell, letölti a SeeMTA Launchert, elindítja, és nézi ha fut, ha meg már nem akkor leáll.

## Mire jó?
Mivel alapesetben a Steam nem érzékeli a SeeMTA-t, így egyedi parancsként rá lehet kötni egy exe-t, amely futásáig úgy érzékeli, hogy fut a játék. Erre van ez kitalálva.

## Hogyan lehet használni?
Igazából baromi egyszerű:
1. [Innen](https://github.com/HVCsano/SzimatLauncher/releases/latest) szedd le az exe-t.
2. Másold be egy neked tetszó mappába.
3. Nyisd meg a Steamen a játék oldalát, majd "Tulajdonságok..." --> Általános --> Indítási opciók
4. Illeszd be oda ezt: `"exe_elérési_útja.exe" %command%`

> Példa: `"C:\SeeMTA_SteamLaunch\szimat_launcher.exe" %command%`

5. Indítsd el **STEAMBÓL VAGY STEAMES PARANCSIKONNAL** a játékot, és el is indul a SeeMTA.
6. Azzal sincs baj, ha nem az ötös pont szerint cselekszel, ha már fut a játék, és úgy indítod el, akkor is érzékelni fogja, nem indítja el mégegyszer.

> Ha persze a sima játékkal akarsz játszani, akkor csak ki kell venned a felső parancsot az opciók közül, és már is megy mint régen.
