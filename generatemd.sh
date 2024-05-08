
# write 10000000 lines of markdown to a file

i=0

while [ $i -lt 10000000 ]; 
do
  echo "## This is a markdown heading" >> test.md
  echo "This is a markdown paragraph" >> test.md
  i=$((i+1))
done