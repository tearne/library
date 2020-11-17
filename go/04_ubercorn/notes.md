go mod init ubercorn

env GOOS=linux GOARCH=arm GOARM=5 go build -o target/ubercorn.arm5 && rsync -aP target/ubercorn.arm5 pi@pi01: