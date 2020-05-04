<?php

$servername = "localhost";
$username = "root";
$password = "Angelamerkelistinwien";
$dbname = "odinseye";

$conn = new mysqli($servername, $username, $password, $dbname);
if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}

if(isset($_GET["exid"])) {

	$sql = "select * from Exercise where pk_id = " . $_GET["exid"];
	// $sql = "select * from Command join CommandList CL on Command.fk_cmd_list_id = CL.pk_id";

	$result = $conn->query($sql);

	$json_exercise;
	
	if ($result->num_rows > 0) {	
	    while($row = $result->fetch_assoc()) {
	        $json_exercise = array(
       			"id" => $row["pk_id"],
       			"name" => $row["name"],
       			"status" => $row["status"],
       			"group_name" => $row["group_name"],
       			"root" => $row["root"],
       			"description" => $row["description"],
       			"attributes" => $row["attributes"]
       		);
	        
    		echo json_encode($json_exercise);
	    }
	} else {
	    echo "0 results";
	}
	$conn->close();

/*
	if(file_exists($path)) {
		header("Content-Type: application/json");
		readfile($path);
	} else {
		die("Error serving file!");
	}
*/
} else {
	include("exercises.html");
}

?>
