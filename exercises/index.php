<?php

$servername = "localhost";
$username = "root";
$password = "Angelamerkelistinwien";
$dbname = "odinseye";
$json_exercise;

$conn = new mysqli($servername, $username, $password, $dbname);

if(!$conn->set_charset("utf8mb4")) {
	$conn->set_charset("utf8mb4");
}
mb_internal_encoding("UTF-8");

if ($conn->connect_error) {
    die("Connection failed: " . $conn->connect_error);
}

if(isset($_GET["exid"])) {

    if ($_GET["exid"]==0) {
	    $sql = "select * from Exercise";
	    $result = $conn->query($sql);

	    $json_exercise = array();
    
    	while ($row = $result->fetch_assoc()) {
    		array_push($json_exercise, array(
                "id" => $row["pk_id"],
                "name" => $row["name"],
                "group_name" => $row["group_name"]
        	));
        }
        echo json_encode($json_exercise);
    } else {

	    $sql = "select Exercise.pk_id, Exercise.name, Exercise.group_name, Exercise.description, Exercise.status, Exercise.root, Exercise.attributes from Exercise where pk_id = " . $_GET["exid"];
		$result = $conn->query($sql);

	    if ($result->num_rows > 0) {
	       while($row = $result->fetch_assoc()) {
	            $json_exercise = array(
	                   "id" => $row["pk_id"],
	                   "name" => $row["name"],
	                   "group_name" => $row["group_name"],
	                   "description" => $row["description"],
	                   "status" => $row["status"],
	                   "root" => $row["root"],
	                   "attributes" => $row["attributes"]
	               );

	            echo json_encode($json_exercise);
	        }
	    } else {
	        echo "0 results";
	    }
	}
}
if(isset($_GET["clid"])) {
	$sql = "select command from Command join CommandList CL on Command.fk_cmd_list_id = CL.pk_id join Exercise E on CL.fk_exercise_id = E.pk_id where E.pk_id = " . $_GET["clid"];
	$result = $conn->query($sql);

	$json_cmdList = array();
	
    if ($result->num_rows > 0) {
       while($row = $result->fetch_assoc()) {
           	array_push($json_cmdList, $row["command"]);
        }
        echo json_encode($json_cmdList);
    } else {
        echo "0 results";
    }
}
if(isset($_GET["stid"])) {
	$sql = "select structure_string from Structure S join Exercise E on S.fk_exercise_id = E.pk_id where E.pk_id = " . $_GET["stid"];
	$result = $conn->query($sql);

	$json_strString;
	
    if ($result->num_rows > 0) {
       while($row = $result->fetch_assoc()) {
           	$json_strString = $row["structure_string"];
       }
       echo $json_strString;
    } else {
       echo "0 results";
    }
}
$conn->close();
?>
