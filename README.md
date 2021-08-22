# README

ただOpenRTB3.0とAdcom1.0を写経してみる

- OpenMedia: https://iabtechlab.com/standards/openmedia/
- OpenRTB3.0: https://github.com/InteractiveAdvertisingBureau/openrtb
- Adcom1.0: https://github.com/InteractiveAdvertisingBureau/openrtb


## 所感

- AdcomのDistibutionChannelあたりが結構害悪。これはOpenRTB3.0のRequest.contextで使われる。
DistributionChannelのsubtypeとして定義されるAppやSiteがcontextに与えられる。
正直そんな抽象化は誰も望んでいないのでは。
最近のAdServerで使われるGolangとかはこのあたりのParserを書くのは簡単なのだろうか(勿論、実装を分ければよいが...)。

