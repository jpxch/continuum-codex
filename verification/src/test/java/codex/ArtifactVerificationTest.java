package codex;

import org.junit.jupiter.api.Test;

import java.io.File;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;

class ArtifactVerificationTest {

    @Test
    void artifact_asses_verification() {
        File artifact = new File("../artifacts/bleach/zanpakuto.json");

        assertDoesNotThrow(() -> {
            Verifier.verify(artifact);
        });
    }
}
