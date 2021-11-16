import java.util.Scanner;

public class ConsoleIO {
	
	public static void main(String[] args) {
		Scanner scanner = new Scanner(System.in);
		System.out.println("Enter some input: ");

		String userInput = scanner.nextLine();

		System.out.println("Your input: " + userInput);
	}
	
}