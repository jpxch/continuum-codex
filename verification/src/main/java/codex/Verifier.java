package codex;

import com.fasterxml.jackson.databind.ObjectMapper;
import java.io.File;
import java.util.HashSet;

public class Verifier {

    public static void main(String[] args) throws Exception {
        ObjectMapper mapper = new ObjectMapper();

        File file = new File("../artifacts/bleach/zanpakuto.json");
        Zanpakuto[] items = mapper.readValue(file, Zanpakuto[].class);

        HashSet<String> ids = new HashSet<>();
        HashSet<String> bankaiJP = new HashSet<>();

        for (Zanpakuto z : items) {

            if (!ids.add(z.id)) {
                throw new RuntimeException("Duplicate id: " + z.id);
            }

            if (z.bankai != null) {
                if (!bankaiJP.add(z.bankai.name.jp)) {
                    throw new RuntimeException("Duplicate Bankai JP: " + z.bankai.name.jp);
                }
            }

            if (z.shikai != null) {
                if (z.shikai.release_command == null) {
                    throw new RuntimeException("Missing release_command.jp for " + z.id);
                }
            }
        }

        System.out.println("Verification passed.");
    }
}