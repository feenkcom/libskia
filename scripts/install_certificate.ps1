$cert_url = "https://mkcert.org/generate/"
$cert_file = New-TemporaryFile

[Net.ServicePointManager]::SecurityProtocol = "tls12, tls11, tls"
Invoke-WebRequest -Uri $cert_url -UseBasicParsing -OutFile $cert_file.FullName

openssl pkcs12 -export -nokeys -out certs.pfx -in $cert_file.FullName -password pass:12345
Import-PfxCertificate -FilePath .\certs.pfx cert:\localMachine\Root -Password (ConvertTo-SecureString "12345" -AsPlainText -Force)
