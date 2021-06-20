import com.google.gson.Gson;

public class Gson {
	
	public static void main(String[] args) {
		Gson gson = new Gson();
		System.out.println(gson.fromJson("1", int.class));
	}
	
}