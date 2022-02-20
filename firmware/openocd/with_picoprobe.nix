{stdenv, automake, which, libftdi1, libusb1, hidapi, libtool, autoconf, git, jimtcl, pkgconfig, fetchFromGitHub}:

stdenv.mkDerivation rec {
  pname = "openocd-picoprobe";
  version = "0.10.0";

  src = fetchFromGitHub {
    owner = "raspberrypi";
    repo = "openocd";
    rev = "df76ec7edee9ebb9be86e4cff7479da642b0e8df";
    sha256 = "0zy29yscbiwprl6pqbqn5arlrxymidgwcbqzh1248b6g62qwj6x3";
  };

  nativeBuildInputs = [ pkgconfig ];
  buildInputs = [ automake which libftdi1 libusb1 hidapi libtool autoconf git jimtcl ];

  preConfigure = ''SKIP_SUBMODULE=1 ./bootstrap'' ;

  configureFlags = [
    "--enable-picoprobe"
    "--disable-internal-jimtcl"
    "--disable-internal-libjaylink"
    #--prefix=/usr/
  ];

  # makeFlags = [ "DESTDIR=$(out)" ];
}
