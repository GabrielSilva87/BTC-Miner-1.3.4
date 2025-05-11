#!/data/data/com.termux/files/usr/bin/sh

pkg update -y && pkg upgrade -y
pkg install -y curl build-essential

if ! command -v rustc > /dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  . $HOME/.cargo/env
fi

if [ ! -f main.rs ]; then
  echo "Arquivo main.rs não encontrado no diretório atual."
  exit 1
fi

rustc main.rs -o bitcoin_generator

echo "Compilação concluída. Execute ./bitcoin_generator para rodar o programa"
