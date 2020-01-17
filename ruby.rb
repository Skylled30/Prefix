puts "Enter expression: "
exp = gets.split
signs = ["+", "-", "/", "*"]
oper = []
check = false
exp.each do |part|
	sign = part.strip
	if sign.length > 0
		if signs.include?(sign)
			oper.push(sign)
		else
			if check
				sign_op = oper.pop
				print sign_op
				print sign
			else
				print sign
				check = true
			end
		end
	end
end
