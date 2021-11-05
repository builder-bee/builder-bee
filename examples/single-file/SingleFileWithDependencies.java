/// com.google.code.gson:gson:2.8.7
import com.google.gson.Gson;

public class SingleFileWithDependencies {

	public static void main(String[] args) {
		Gson gson = new Gson();
		System.out.println(gson.fromJson("1", int.class));
	}
	
}
