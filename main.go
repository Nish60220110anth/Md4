package main

import (
	"bufio"
	"bytes"
	"fmt"
	"log"
	"os"
	"os/exec"
	"strings"
)

func CheckError(err error) {
	if err != nil {
		log.Fatalln(err.Error())
	}
}

func GetDigest(msg string) string {
	file, err := os.CreateTemp("", "input")
	CheckError(err)
	_, err = file.Write([]byte(msg))
	CheckError(err)
	cmd := exec.Command("openssl", "dgst", "-r", "--md4", file.Name())
	data, _ := cmd.Output()
	CheckError(err)
	data = bytes.Split(data, []byte(" "))[0]
	return string(data)
}

func RunMd4(infile, outfile string) {
	cmd := exec.Command("./md4", infile, outfile)
	err := cmd.Run()

	if err != nil {
		log.Fatalf("Failed to generate md4 output\nProcess exited with error: %s", err.Error())
	}
}

func CreateFile(fln string) *os.File {
	fl, err := os.Create(fln)

	if per, ok := err.(*os.PathError); ok {
		if per.Err == os.ErrInvalid {
			log.Fatalf("file name: %s is invalid", fln)
			return nil
		} else {
			log.Fatalf("Error encountered while creating file\nExited with error: %s", per.Error())
			return nil
		}
	}

	return fl
}

type Stats struct {
	Inputs       []string
	Outputs      []string
	Inputflname  string // input file name
	Outflname    string // output file name for original digest
	Outflgenname string // testing file name for digest
	Other        string // other stats
}

func (st *Stats) AddInput(in string) {
	st.Inputs = append(st.Inputs, in)
}

func (st *Stats) AddOutput(out string) {
	st.Outputs = append(st.Outputs, out)
}

func (st *Stats) PrintStats() {
	fl, _ := os.Create("stats.txt")
	bf := bufio.NewWriter(fl)

	bf.WriteString(fmt.Sprintf("Input file name: %s\n", st.Inputflname))
	bf.WriteString(fmt.Sprintf("Output file name: %s\n", st.Outflname))
	bf.WriteString(fmt.Sprintf("Output file name for generated digest: %s\n", st.Outflgenname))
	bf.WriteString(fmt.Sprintf("Input line count: %d\n", len(st.Inputs)))
	bf.WriteString("Pairwise input-output\n")
	bf.WriteString("Input\t\tOutput\n")
	for i, input := range st.Inputs {
		bf.WriteString(fmt.Sprintf("%s\t%s\n", input, st.Outputs[i]))
	}
	bf.WriteString(fmt.Sprintf("Other: %s\n", st.Other))
	bf.Flush()
}

func main() {

	if len(os.Args) != 1 {
		fmt.Println("Arguments are not required")
	}

	var st *Stats = new(Stats)

	var inflname, outflname string
	fmt.Print("Enter input file name: ")
	fmt.Scanf("%s\n", &inflname)
	fmt.Print("Enter out file name: ")
	fmt.Scanf("%s\n", &outflname)

	st.Inputflname = inflname
	st.Outflname = outflname
	st.Other = "openssl used to get the correct value of md4 digest for inputs"

	infl, _ := os.Open(inflname)
	outfl := CreateFile(outflname)

	outbf := bufio.NewWriter(outfl)

	var inputs []string
	inscanner := bufio.NewScanner(infl)

	for inscanner.Scan() {
		inputs = append(inputs, inscanner.Text())
	}

	st.Inputs = inputs

	outsl := strings.Split(outflname, ".")
	outflname1 := outsl[0]
	outext := outsl[1]

	outflgen := fmt.Sprintf("%s_gen.%s", outflname1, outext)
	st.Outflgenname = outflgen
	RunMd4(inflname, outflgen)

	for _, input := range inputs {
		digest := GetDigest(input)
		st.AddOutput(digest)
		con := fmt.Sprintln(fmt.Sprintf("%s : %s", input, digest))

		if outbf.Available() != 0 {
			outbf.WriteString(con)
		} else {
			outbf.Flush()
			outbf.WriteString(con)
		}
	}

	outbf.Flush()

	outfile, _ := os.Open(outflname)
	outgenfile, _ := os.Open(outflgen)
	noutbf := bufio.NewReader(outfile)
	noutgen := bufio.NewReader(outgenfile)

	for {
		line1, _, err1 := noutbf.ReadLine()
		line2, _, err2 := noutgen.ReadLine()

		line1s := string(line1)
		line2s := string(line2)

		if line1s != line2s {
			log.Fatalln("file content not equal")
		} else if err1 != nil || err2 != nil {
			break
		}
	}

	st.PrintStats()
	fmt.Println("File contents are equal! Success!")
}
