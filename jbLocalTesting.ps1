# https://stackoverflow.com/a/28482050
$qemu = Get-Process qemu-system-x86_64 -ErrorAction SilentlyContinue
if ($qemu) {
  if (!$qemu.HasExited) {
    $qemu | Stop-Process -Force
  }
}
Remove-Variable qemu

docker run --rm -it -v "${pwd}:/root/env" jos_buildenv make -B build-x86_64
qemu-system-x86_64 -cdrom dist/x86_64/kernel.iso