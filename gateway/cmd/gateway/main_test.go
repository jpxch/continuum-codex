package main

import (
	"encoding/json"
	"os"
	"testing"
)

func TestArtifactContractShape(t *testing.T) {
	data, err := os.ReadFile("../../../artifacts/bleach/zanpakuto.json")
	if err != nil {
		t.Fatalf("failed to read artifact: %v", err)
	}

	var items []Zanpakuto
	if err := json.Unmarshal(data, &items); err != nil {
		t.Fatalf("artifact does not match expected shape: %v", err)
	}

	if len(items) == 0 {
		t.Fatal("artifact is empty")
	}

	if items[0].Shikai == nil {
		t.Fatal("missing shikai")
	}

	if items[0].Shikai.ReleaseCommand.Jp == "" {
		t.Fatal("release_command.jp missing")
	}
}