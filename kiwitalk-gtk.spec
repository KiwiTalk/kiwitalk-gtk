Name:		kiwitalk-gtk
Version: 	0.1.0
Release:	1
BuildRequires:	cargo

%prep
cargo update

%build
cargo build --release

%install
echo install todo	
