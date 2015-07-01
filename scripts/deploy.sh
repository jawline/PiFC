#Assumes $1 is file to deploy, $2 is the addr of the remote
sftp $2 -e "put $1; bye"
ssh $2 "echo hi"
